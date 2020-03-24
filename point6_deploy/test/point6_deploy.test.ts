import { expect as expectCDK, matchTemplate, MatchStyle } from '@aws-cdk/assert';
import * as cdk from '@aws-cdk/core';
import Point6Deploy = require('../lib/point6_deploy-stack');

test('Empty Stack', () => {
    const app = new cdk.App();
    // WHEN
    const stack = new Point6Deploy.Point6DeployStack(app, 'MyTestStack');
    // THEN
    expectCDK(stack).to(matchTemplate({
      "Resources": {}
    }, MatchStyle.EXACT))
});
