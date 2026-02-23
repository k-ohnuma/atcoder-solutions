export type TerraformVariableType = "string";

export interface StringVariableDefinition {
  tfName: string;
  type: TerraformVariableType;
  defaultValue?: string;
  sensitive?: boolean;
}

export interface BackendStackVariableDefinitions {
  environment: StringVariableDefinition;
  projectId: StringVariableDefinition;
  region: StringVariableDefinition;
  apiServiceName: StringVariableDefinition;
  dailyJobName: StringVariableDefinition;
  apiContainerImage: StringVariableDefinition;
  appDatabaseUrl: StringVariableDefinition;
  firebaseProjectId: StringVariableDefinition;
  atcoderProblemsBaseEndpoint: StringVariableDefinition;
  dailyCron: StringVariableDefinition;
  dailyTimeZone: StringVariableDefinition;
}

export const backendStackVariableDefinitions: BackendStackVariableDefinitions = {
  environment: {
    tfName: "environment",
    type: "string",
    defaultValue: "dev",
  },
  projectId: {
    tfName: "project_id",
    type: "string",
    defaultValue: 'atcoder-solutions'
  },
  region: {
    tfName: "region",
    type: "string",
    defaultValue: "asia-northeast1",
  },
  apiServiceName: {
    tfName: "api_service_name",
    type: "string",
    defaultValue: "atcoder-solutions-api",
  },
  dailyJobName: {
    tfName: "daily_job_name",
    type: "string",
    defaultValue: "atcoder-solutions-daily",
  },
  apiContainerImage: {
    tfName: "api_container_image",
    type: "string",
  },
  appDatabaseUrl: {
    tfName: "app_database_url",
    type: "string",
    sensitive: true,
  },
  firebaseProjectId: {
    tfName: "firebase_project_id",
    type: "string",
    defaultValue: "atcoder-solutions",
  },
  atcoderProblemsBaseEndpoint: {
    tfName: "atcoder_problems_base_endpoint",
    type: "string",
    defaultValue: "https://kenkoooo.com/atcoder",
  },
  dailyCron: {
    tfName: "daily_cron",
    type: "string",
    defaultValue: "0 5 * * *",
  },
  dailyTimeZone: {
    tfName: "daily_time_zone",
    type: "string",
    defaultValue: "Asia/Tokyo",
  },
};

export interface BackendStackStaticConfig {
  requiredProjectServices: readonly string[];
  runServiceAccountIdBase: string;
  schedulerServiceAccountIdBase: string;
  appDatabaseSecretIdBase: string;
  schedulerJobNameBase: string;
  runDailyJobArg: string;
  runtimeEnv: string;
  schedulerOidcAudience: string;
}

export const backendStackStaticConfig: BackendStackStaticConfig = {
  requiredProjectServices: [
    "run.googleapis.com",
    "cloudscheduler.googleapis.com",
    "secretmanager.googleapis.com",
    "iam.googleapis.com",
  ],
  runServiceAccountIdBase: "atcoder-solutions-run",
  schedulerServiceAccountIdBase: "atcoder-solutions-scheduler",
  appDatabaseSecretIdBase: "app-database-url",
  schedulerJobNameBase: "atcoder-solutions-daily-import",
  runDailyJobArg: "run_daily_job",
  runtimeEnv: "prd",
  schedulerOidcAudience: "https://run.googleapis.com/",
};
