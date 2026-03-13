# \ConfigurationApi

All URIs are relative to *http://localhost:8000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**allauth_config_get**](ConfigurationApi.md#allauth_config_get) | **GET** /api/auth/v1/config | Get configuration



## allauth_config_get

> models::AllauthConfigurationResponse allauth_config_get()
Get configuration

There are many configuration options that alter the functionality and behavior of django-allauth, some of which can also impact the frontend. Therefore, relevant configuration options are exposed via this endpoint. The data returned is not user/authentication dependent. Hence, it suffices to only fetch this data once at boot time of your application. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::AllauthConfigurationResponse**](allauth.ConfigurationResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

