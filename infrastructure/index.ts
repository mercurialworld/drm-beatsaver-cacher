import * as cdk from "aws-cdk-lib";
import { BeatSaverCacherStack } from "./stack.ts";

const app = new cdk.App();
new BeatSaverCacherStack(app, "BeatSaverCacherStack", {
  env: { account: "575108959833", region: "us-east-1" },
});