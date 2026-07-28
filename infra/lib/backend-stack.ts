import { GcsBackend, TerraformOutput, TerraformStack, TerraformVariable } from "cdktn";
import { Construct } from "constructs";
import { CloudRunServiceIamMember } from "../.gen/providers/google/cloud-run-service-iam-member";
import { CloudRunV2Job } from "../.gen/providers/google/cloud-run-v2-job";
import { CloudRunV2JobIamMember } from "../.gen/providers/google/cloud-run-v2-job-iam-member";
import { CloudRunV2Service } from "../.gen/providers/google/cloud-run-v2-service";
import { CloudSchedulerJob } from "../.gen/providers/google/cloud-scheduler-job";
import { ProjectIamCustomRole } from "../.gen/providers/google/project-iam-custom-role";
import { ProjectIamMember } from "../.gen/providers/google/project-iam-member";
import { ProjectService } from "../.gen/providers/google/project-service";
import { GoogleProvider } from "../.gen/providers/google/provider";
import { SecretManagerSecret } from "../.gen/providers/google/secret-manager-secret";
import { SecretManagerSecretIamMember } from "../.gen/providers/google/secret-manager-secret-iam-member";
import { ServiceAccount } from "../.gen/providers/google/service-account";
import {
  backendStackStaticConfig,
  backendStackVariableDefinitions,
  type StringVariableDefinition,
  stackEnvironmentName,
} from "../config/config";

export class BackendStack extends TerraformStack {
  constructor(scope: Construct, id: string) {
    super(scope, id);
    new GcsBackend(this, {
      bucket: backendStackStaticConfig.tfStateBackendBucket,
      prefix: `${backendStackStaticConfig.tfStateBackendPrefixBase}/${stackEnvironmentName}`,
    });

    const defineStringVariable = (definition: StringVariableDefinition): TerraformVariable =>
      new TerraformVariable(this, definition.tfName, {
        type: definition.type,
        default: definition.defaultValue,
        sensitive: definition.sensitive,
      });

    const projectId = defineStringVariable(backendStackVariableDefinitions.projectId);
    const environment = defineStringVariable(backendStackVariableDefinitions.environment);
    const region = defineStringVariable(backendStackVariableDefinitions.region);
    const apiServiceName = defineStringVariable(backendStackVariableDefinitions.apiServiceName);
    const dailyJobName = defineStringVariable(backendStackVariableDefinitions.dailyJobName);
    const apiContainerImage = defineStringVariable(backendStackVariableDefinitions.apiContainerImage);
    const appDatabaseUrlSecretVersion = defineStringVariable(backendStackVariableDefinitions.appDatabaseUrlSecretVersion);
    const turnstileSecretKeyVersion = defineStringVariable(backendStackVariableDefinitions.turnstileSecretKeyVersion);
    const firebaseProjectId = defineStringVariable(backendStackVariableDefinitions.firebaseProjectId);
    const atcoderProblemsBaseEndpoint = defineStringVariable(backendStackVariableDefinitions.atcoderProblemsBaseEndpoint);
    const atcoderDifficultyBaseEndpoint = defineStringVariable(backendStackVariableDefinitions.atcoderDifficultyBaseEndpoint);
    const dailyCron = defineStringVariable(backendStackVariableDefinitions.dailyCron);
    const dailyTimeZone = defineStringVariable(backendStackVariableDefinitions.dailyTimeZone);

    const withEnvSuffix = (base: string): string => `${base}-${environment.value}`;

    const apiServiceResourceName = withEnvSuffix(apiServiceName.value);
    const dailyJobResourceName = withEnvSuffix(dailyJobName.value);
    const schedulerJobResourceName = withEnvSuffix(backendStackStaticConfig.schedulerJobNameBase);
    const runServiceAccountId = withEnvSuffix(backendStackStaticConfig.runServiceAccountIdBase);
    const schedulerServiceAccountId = withEnvSuffix(backendStackStaticConfig.schedulerServiceAccountIdBase);
    const appDatabaseSecretId = withEnvSuffix(backendStackStaticConfig.appDatabaseSecretIdBase);
    const turnstileSecretId = withEnvSuffix(backendStackStaticConfig.turnstileSecretIdBase);

    new GoogleProvider(this, "google", {
      project: projectId.value,
      region: region.value,
    });

    const serviceEnables = backendStackStaticConfig.requiredProjectServices.map(
      (service) =>
        new ProjectService(this, `api-${service.replace(/\./g, "-")}`, {
          project: projectId.value,
          service,
          disableOnDestroy: true,
        }),
    );

    const runServiceAccount = new ServiceAccount(this, "run-service-account", {
      project: projectId.value,
      accountId: runServiceAccountId,
      displayName: "AtCoder Solutions Run",
      dependsOn: serviceEnables,
    });

    const schedulerServiceAccount = new ServiceAccount(this, "scheduler-service-account", {
      project: projectId.value,
      accountId: schedulerServiceAccountId,
      displayName: "AtCoder Solutions Scheduler",
      dependsOn: serviceEnables,
    });

    const appDatabaseUrlSecret = new SecretManagerSecret(this, "app-database-url-secret", {
      project: projectId.value,
      secretId: appDatabaseSecretId,
      replication: { auto: {} },
      dependsOn: serviceEnables,
    });

    const turnstileSecretKeySecret = new SecretManagerSecret(this, "turnstile-secret-key-secret", {
      project: projectId.value,
      secretId: turnstileSecretId,
      replication: { auto: {} },
      dependsOn: serviceEnables,
    });

    const firebaseAuthUserManagerRole = new ProjectIamCustomRole(this, "firebase-auth-user-manager-role", {
      project: firebaseProjectId.value,
      roleId: `atcoderSolutionsFirebaseAuthUserManager${environment.value}`,
      title: "AtCoder Solutions Firebase Auth User Manager",
      description: "Allows the backend service account to create and delete Firebase Authentication users.",
      permissions: ["firebaseauth.users.create", "firebaseauth.users.delete"],
    });

    new ProjectIamMember(this, "auth-user-manager", {
      project: firebaseProjectId.value,
      role: firebaseAuthUserManagerRole.name,
      member: `serviceAccount:${runServiceAccount.email}`,
      dependsOn: [firebaseAuthUserManagerRole],
    });

    new SecretManagerSecretIamMember(this, "run-sa-secret-access", {
      project: projectId.value,
      secretId: appDatabaseUrlSecret.secretId,
      role: "roles/secretmanager.secretAccessor",
      member: `serviceAccount:${runServiceAccount.email}`,
    });

    new SecretManagerSecretIamMember(this, "run-sa-turnstile-secret-access", {
      project: projectId.value,
      secretId: turnstileSecretKeySecret.secretId,
      role: "roles/secretmanager.secretAccessor",
      member: `serviceAccount:${runServiceAccount.email}`,
    });

    const service = new CloudRunV2Service(this, "api-service", {
      project: projectId.value,
      location: region.value,
      name: apiServiceResourceName,
      ingress: "INGRESS_TRAFFIC_ALL",
      deletionProtection: false,
      template: {
        serviceAccount: runServiceAccount.email,
        containers: [
          {
            image: apiContainerImage.value,
            env: [
              {
                name: "APP_DATABASE_URL",
                valueSource: {
                  secretKeyRef: {
                    secret: appDatabaseUrlSecret.secretId,
                    version: appDatabaseUrlSecretVersion.value,
                  },
                },
              },
              {
                name: "FIREBASE_PROJECT_ID",
                value: firebaseProjectId.value,
              },
              {
                name: "TURNSTILE_ENABLED",
                value: "true",
              },
              {
                name: "TURNSTILE_SECRET_KEY",
                valueSource: {
                  secretKeyRef: {
                    secret: turnstileSecretKeySecret.secretId,
                    version: turnstileSecretKeyVersion.value,
                  },
                },
              },
              {
                name: "ENV",
                value: backendStackStaticConfig.runtimeEnv,
              },
              {
                name: "ATCODER_PROBLEMS_BASE_ENDPOINT",
                value: atcoderProblemsBaseEndpoint.value,
              },
              {
                name: "ATCODER_PROBLEMS_DIFFICULTY_ENDPOINT",
                value: atcoderDifficultyBaseEndpoint.value,
              },
            ],
          },
        ],
      },
      dependsOn: [runServiceAccount],
    });

    new CloudRunServiceIamMember(this, "api-service-public-invoker", {
      project: projectId.value,
      location: region.value,
      service: apiServiceResourceName,
      role: "roles/run.invoker",
      member: "allUsers",
      dependsOn: [service],
    });

    const dailyImportJob = new CloudRunV2Job(this, "daily-import-job", {
      project: projectId.value,
      location: region.value,
      name: dailyJobResourceName,
      deletionProtection: false,
      template: {
        template: {
          timeout: backendStackStaticConfig.dailyJobTimeout,
          serviceAccount: runServiceAccount.email,
          containers: [
            {
              image: apiContainerImage.value,
              command: [...backendStackStaticConfig.runDailyJobCommand],
              env: [
                {
                  name: "APP_DATABASE_URL",
                  valueSource: {
                    secretKeyRef: {
                      secret: appDatabaseUrlSecret.secretId,
                      version: appDatabaseUrlSecretVersion.value,
                    },
                  },
                },
                {
                  name: "FIREBASE_PROJECT_ID",
                  value: firebaseProjectId.value,
                },
                {
                  name: "ENV",
                  value: backendStackStaticConfig.runtimeEnv,
                },
                {
                  name: "ATCODER_PROBLEMS_BASE_ENDPOINT",
                  value: atcoderProblemsBaseEndpoint.value,
                },
                {
                  name: "ATCODER_PROBLEMS_DIFFICULTY_ENDPOINT",
                  value: atcoderDifficultyBaseEndpoint.value,
                },
              ],
            },
          ],
        },
      },
      dependsOn: [runServiceAccount],
    });

    new ProjectService(this, "cloud-run-jobs-api", {
      project: projectId.value,
      service: "run.googleapis.com",
      disableOnDestroy: false,
    });

    new CloudRunV2JobIamMember(this, "scheduler-job-invoker", {
      project: projectId.value,
      location: region.value,
      name: dailyImportJob.name,
      role: "roles/run.invoker",
      member: `serviceAccount:${schedulerServiceAccount.email}`,
    });

    new CloudSchedulerJob(this, "daily-import-scheduler", {
      project: projectId.value,
      region: region.value,
      name: schedulerJobResourceName,
      schedule: dailyCron.value,
      timeZone: dailyTimeZone.value,
      httpTarget: {
        uri: `https://run.googleapis.com/v2/projects/${projectId.value}/locations/${region.value}/jobs/${dailyJobResourceName}:run`,
        httpMethod: "POST",
        oauthToken: {
          serviceAccountEmail: schedulerServiceAccount.email,
        },
      },
      dependsOn: [dailyImportJob],
    });

    new TerraformOutput(this, "api_service_uri", {
      value: service.uri,
      description: "api_endpoint",
    });
    new TerraformOutput(this, "daily_job_name_output", {
      value: dailyImportJob.name,
      description: "Cloud Run Job name",
    });
  }
}
