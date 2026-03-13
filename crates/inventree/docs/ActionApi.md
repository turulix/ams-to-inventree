# \ActionApi

All URIs are relative to *https://inv.turulix.de*

Method | HTTP request | Description
------------- | ------------- | -------------
[**action_create**](ActionApi.md#action_create) | **POST** /api/action/ | 



## action_create

> models::ActionPlugin action_create(action_plugin)


This function checks if all required info was submitted and then performs a plugin_action or returns an error.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**action_plugin** | [**ActionPlugin**](ActionPlugin.md) |  | [required] |

### Return type

[**models::ActionPlugin**](ActionPlugin.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

