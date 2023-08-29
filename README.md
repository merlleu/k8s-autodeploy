# k8s-autodeploy

## Description
This is a simple script to redeploy a kubernetes deployment. It is intended to be used in a CI/CD pipeline to redeploy a deployment after a new image has been built.
It uses Rancher API to do it after a webhook has been triggered.

## Environment variables
- RANCHER_URL: Rancher URL
- RANCHER_TOKEN: Rancher access key
- RANCHER_CLUSTER_ID: Rancher cluster ID
- WEBHOOK_SECRET: Secret to be used in the webhook

## Use with docker
- `docker pull merlleu/k8s-autodeploy:latest`
- ```bash
docker run 
    -d 
    -e RANCHER_URL=a 
    -e RANCHER_TOKEN=a 
    -e RANCHER_CLUSTER_ID=a 
    -e WEBHOOK_SECRET="Bearer xyz" 
    merlleu/k8s-autodeploy:latest`