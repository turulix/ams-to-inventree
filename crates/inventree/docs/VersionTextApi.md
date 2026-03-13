# \VersionTextApi

All URIs are relative to *http://localhost:8000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**version_text_list**](VersionTextApi.md#version_text_list) | **GET** /api/version-text | 



## version_text_list

> Vec<models::VersionInformation> version_text_list(start_version, versions)


Simple JSON endpoint for InvenTree version text.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_version** | Option<**i32**> | First version to report. Defaults to return the latest {versions} versions. |  |
**versions** | Option<**i32**> | Number of versions to return. |  |[default to 10]

### Return type

[**Vec<models::VersionInformation>**](VersionInformation.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

