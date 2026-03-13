#!/bin/bash

$TargetDir = "crates/inventree"

if (Test-Path "$TargetDir/src") {
    Remove-Item -Path "$TargetDir/src" -Recurse -Force
}

docker run --rm -v "$($PWD.Path):/local" openapitools/openapi-generator-cli generate `
    -i https://raw.githubusercontent.com/inventree/schema/refs/heads/main/export/453/api.yaml `
    -g rust `
    --additional-properties=packageName=inventree `
    --additional-properties=library=reqwest-trait `
    --additional-properties=topLevelApiClient=true `
    --additional-properties=useSingleRequestParameter=true `
    --additional-properties=reqwestDefaultFeatures=rustls `
    --additional-properties=preferUnsignedInt=true `
    --additional-properties=bestFitInt=true `
    --additional-properties=useBonBuilder=true `
    -o "/local/$TargetDir"

cargo fmt -p inventree