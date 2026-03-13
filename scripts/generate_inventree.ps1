#!/bin/bash

$TargetDir = "crates/inventree"

docker run --rm -v "$($PWD.Path):/local" openapitools/openapi-generator-cli generate `
    -i https://inv.turulix.de/api/schema/ `
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