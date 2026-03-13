# \LocateApi

All URIs are relative to *http://localhost:8000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**locate_create**](LocateApi.md#locate_create) | **POST** /api/locate/ | 



## locate_create

> models::LocatePlugin locate_create(locate_plugin)


Identify or 'locate' a stock item or location with a plugin.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**locate_plugin** | [**LocatePlugin**](LocatePlugin.md) |  | [required] |

### Return type

[**models::LocatePlugin**](LocatePlugin.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

