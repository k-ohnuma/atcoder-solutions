import { App } from "cdktf";
import { BackendStack } from "../lib/backend-stack";

const app = new App();
const projectName = `atcoder-solutions-backend-infra-stack`;

new BackendStack(app, projectName);
app.synth();
