# \ParameterApi

All URIs are relative to *http://localhost:8000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**parameter_bulk_destroy**](ParameterApi.md#parameter_bulk_destroy) | **DELETE** /api/parameter/ | 
[**parameter_create**](ParameterApi.md#parameter_create) | **POST** /api/parameter/ | 
[**parameter_destroy**](ParameterApi.md#parameter_destroy) | **DELETE** /api/parameter/{id}/ | 
[**parameter_list**](ParameterApi.md#parameter_list) | **GET** /api/parameter/ | 
[**parameter_partial_update**](ParameterApi.md#parameter_partial_update) | **PATCH** /api/parameter/{id}/ | 
[**parameter_retrieve**](ParameterApi.md#parameter_retrieve) | **GET** /api/parameter/{id}/ | 
[**parameter_template_create**](ParameterApi.md#parameter_template_create) | **POST** /api/parameter/template/ | 
[**parameter_template_destroy**](ParameterApi.md#parameter_template_destroy) | **DELETE** /api/parameter/template/{id}/ | 
[**parameter_template_list**](ParameterApi.md#parameter_template_list) | **GET** /api/parameter/template/ | 
[**parameter_template_partial_update**](ParameterApi.md#parameter_template_partial_update) | **PATCH** /api/parameter/template/{id}/ | 
[**parameter_template_retrieve**](ParameterApi.md#parameter_template_retrieve) | **GET** /api/parameter/template/{id}/ | 
[**parameter_template_update**](ParameterApi.md#parameter_template_update) | **PUT** /api/parameter/template/{id}/ | 
[**parameter_update**](ParameterApi.md#parameter_update) | **PUT** /api/parameter/{id}/ | 



## parameter_bulk_destroy

> parameter_bulk_destroy(bulk_request)


Perform a DELETE operation against this list endpoint.  Note that the typical DRF list endpoint does not support DELETE, so this method is provided as a custom implementation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bulk_request** | [**BulkRequest**](BulkRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## parameter_create

> models::Parameter parameter_create(parameter)


List API endpoint for Parameter objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**parameter** | [**Parameter**](Parameter.md) |  | [required] |

### Return type

[**models::Parameter**](Parameter.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## parameter_destroy

> parameter_destroy(id)


Detail API endpoint for Parameter objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## parameter_list

> models::PaginatedParameterList parameter_list(limit, enabled, model_id, model_type, offset, ordering, search, template, updated_by)


Override the GET method to determine export options.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | **i32** | Number of results to return per page. | [required] |
**enabled** | Option<**bool**> | Template Enabled |  |
**model_id** | Option<**i32**> |  |  |
**model_type** | Option<**String**> | Model Type |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**search** | Option<**String**> | A search term. Searched fields: data, template__description, template__name, template__units. |  |
**template** | Option<**i32**> |  |  |
**updated_by** | Option<**i32**> |  |  |

### Return type

[**models::PaginatedParameterList**](PaginatedParameterList.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## parameter_partial_update

> models::Parameter parameter_partial_update(id, patched_parameter)


Detail API endpoint for Parameter objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**patched_parameter** | Option<[**PatchedParameter**](PatchedParameter.md)> |  |  |

### Return type

[**models::Parameter**](Parameter.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## parameter_retrieve

> models::Parameter parameter_retrieve(id)


Detail API endpoint for Parameter objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::Parameter**](Parameter.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## parameter_template_create

> models::ParameterTemplate parameter_template_create(parameter_template)


List view for ParameterTemplate objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**parameter_template** | [**ParameterTemplate**](ParameterTemplate.md) |  | [required] |

### Return type

[**models::ParameterTemplate**](ParameterTemplate.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## parameter_template_destroy

> parameter_template_destroy(id)


Detail view for a ParameterTemplate object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## parameter_template_list

> models::PaginatedParameterTemplateList parameter_template_list(limit, checkbox, enabled, exists_for_model, for_model, has_choices, has_units, model_type, name, offset, ordering, search, units)


Override the GET method to determine export options.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | **i32** | Number of results to return per page. | [required] |
**checkbox** | Option<**bool**> |  |  |
**enabled** | Option<**bool**> |  |  |
**exists_for_model** | Option<**String**> | Exists For Model |  |
**for_model** | Option<**String**> | For Model |  |
**has_choices** | Option<**bool**> | Has Choice |  |
**has_units** | Option<**bool**> | Has Units |  |
**model_type** | Option<**String**> | Model Type |  |
**name** | Option<**String**> |  |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**search** | Option<**String**> | A search term. Searched fields: description, name. |  |
**units** | Option<**String**> |  |  |

### Return type

[**models::PaginatedParameterTemplateList**](PaginatedParameterTemplateList.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## parameter_template_partial_update

> models::ParameterTemplate parameter_template_partial_update(id, patched_parameter_template)


Detail view for a ParameterTemplate object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**patched_parameter_template** | Option<[**PatchedParameterTemplate**](PatchedParameterTemplate.md)> |  |  |

### Return type

[**models::ParameterTemplate**](ParameterTemplate.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## parameter_template_retrieve

> models::ParameterTemplate parameter_template_retrieve(id)


Detail view for a ParameterTemplate object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::ParameterTemplate**](ParameterTemplate.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## parameter_template_update

> models::ParameterTemplate parameter_template_update(id, parameter_template)


Detail view for a ParameterTemplate object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**parameter_template** | [**ParameterTemplate**](ParameterTemplate.md) |  | [required] |

### Return type

[**models::ParameterTemplate**](ParameterTemplate.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## parameter_update

> models::Parameter parameter_update(id, parameter)


Detail API endpoint for Parameter objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**parameter** | [**Parameter**](Parameter.md) |  | [required] |

### Return type

[**models::Parameter**](Parameter.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

