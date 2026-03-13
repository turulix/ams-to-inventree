# \EmailApi

All URIs are relative to *https://inv.turulix.de*

Method | HTTP request | Description
------------- | ------------- | -------------
[**email_generate_create**](EmailApi.md#email_generate_create) | **POST** /api/email/generate/ | 



## email_generate_create

> models::GetSimpleLogin email_generate_create(get_simple_login)


Get the token for the current user or fail.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_simple_login** | [**GetSimpleLogin**](GetSimpleLogin.md) |  | [required] |

### Return type

[**models::GetSimpleLogin**](GetSimpleLogin.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

