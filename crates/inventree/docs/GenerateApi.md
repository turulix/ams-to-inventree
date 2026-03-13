# \GenerateApi

All URIs are relative to *https://inv.turulix.de*

Method | HTTP request | Description
------------- | ------------- | -------------
[**generate_batch_code_create**](GenerateApi.md#generate_batch_code_create) | **POST** /api/generate/batch-code/ | 
[**generate_serial_number_create**](GenerateApi.md#generate_serial_number_create) | **POST** /api/generate/serial-number/ | 



## generate_batch_code_create

> models::GenerateBatchCode generate_batch_code_create(generate_batch_code)


Generate a new batch code.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**generate_batch_code** | Option<[**GenerateBatchCode**](GenerateBatchCode.md)> |  |  |

### Return type

[**models::GenerateBatchCode**](GenerateBatchCode.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## generate_serial_number_create

> models::GenerateSerialNumber generate_serial_number_create(generate_serial_number)


Generate a new serial number.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**generate_serial_number** | Option<[**GenerateSerialNumber**](GenerateSerialNumber.md)> |  |  |

### Return type

[**models::GenerateSerialNumber**](GenerateSerialNumber.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

