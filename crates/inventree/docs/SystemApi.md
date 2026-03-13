# \SystemApi

All URIs are relative to *http://localhost:8000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**system_health_retrieve**](SystemApi.md#system_health_retrieve) | **GET** /api/system/health/ | 



## system_health_retrieve

> models::HealthCheckStatus system_health_retrieve()


Simple health check endpoint for monitoring purposes.  Use the root API endpoint for more detailed information (using an authenticated request).

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::HealthCheckStatus**](HealthCheckStatus.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

