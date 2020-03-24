#!/usr/bin/env node
import 'source-map-support/register';
import * as cdk from '@aws-cdk/core';
import { Point6DeployStack } from '../lib/point6_deploy-stack';

const app = new cdk.App();
new Point6DeployStack(app, 'Point6DeployStack');
