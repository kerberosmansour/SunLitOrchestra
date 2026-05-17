import * as pulumi from "@pulumi/pulumi";
import { aws } from "@hulumi/baseline";

const bucket = new aws.SecureBucket("audit-log-bucket", {
  forceDestroy: false,
  versioning: true,
  blockPublicAccess: true,
  encryption: {
    mode: "kms",
  },
});

export const bucketName: pulumi.Output<string> = bucket.bucketName;
