import * as cdk from '@aws-cdk/core';
import * as apigateway from "@aws-cdk/aws-apigateway";
import * as sns from "@aws-cdk/aws-sns";
import * as subs from "@aws-cdk/aws-sns-subscriptions";
import * as lambda from "@aws-cdk/aws-lambda";
import * as iam from "@aws-cdk/aws-iam";
import * as secretManager from "@aws-cdk/aws-secretsmanager"
import * as dynamodb from "@aws-cdk/aws-dynamodb"

export class Point6DeployStack extends cdk.Stack {
  constructor(scope: cdk.Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    const signingSecret = new secretManager.Secret(this, "SlackSigningSecret")

    const slackIncomingHandler = new lambda.Function(this, "BigHeroPoint6", {
      runtime: lambda.Runtime.PROVIDED,
      code: lambda.Code.fromAsset("../target/x86_64-unknown-linux-musl/release/slack_incoming_handler.zip"),
      handler: "ignored"
    })
    signingSecret.grantRead(slackIncomingHandler)

    slackIncomingHandler.role?.addToPolicy(new iam.PolicyStatement({
      effect: iam.Effect.ALLOW,
      resources: ["arn:aws:sns:*"],
      actions: ['SNS:ListTopics']
    }))

    const api = new apigateway.RestApi(this, "BigHeroPoint6-api", {
      restApiName: "BigHeroPoint6API",
      description: "Slack Bot."
    });

    const slackIntegration = new apigateway.LambdaIntegration(slackIncomingHandler, {
      requestTemplates: { "application/json": '{ "statusCode": "200" }' }
    });

    api.root.addResource("slack").addMethod("POST", slackIntegration);

    const messages = new sns.Topic(this, "slack_messages", {
      displayName: "BigHeroPoint6Incoming",
      topicName: "slack_incoming_messages"
    });

    messages.grantPublish(slackIncomingHandler)

    const clientSecret = new secretManager.Secret(this, "SlackClientSecret")

    const echoTable = new dynamodb.Table(this, "BigHeroEchoTable", {
      partitionKey: { type: dynamodb.AttributeType.STRING, name: "SlackUserId" },
      tableName: "bhp6_echo_v1",
    })

    const bigHeroEcho = new lambda.Function(this, "BigHeroEcho", {
      runtime: lambda.Runtime.PROVIDED,
      code: lambda.Code.fromAsset("../target/x86_64-unknown-linux-musl/release/big_hero_echo.zip"),
      handler: "ignored"
    })
    clientSecret.grantRead(bigHeroEcho)
    echoTable.grantReadWriteData(bigHeroEcho)

    messages.addSubscription(new subs.LambdaSubscription(bigHeroEcho, {}));

  }
}
