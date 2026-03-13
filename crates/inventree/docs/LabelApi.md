# \LabelApi

All URIs are relative to *http://localhost:8000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**label_print_create**](LabelApi.md#label_print_create) | **POST** /api/label/print/ | 
[**label_template_create**](LabelApi.md#label_template_create) | **POST** /api/label/template/ | 
[**label_template_destroy**](LabelApi.md#label_template_destroy) | **DELETE** /api/label/template/{id}/ | 
[**label_template_list**](LabelApi.md#label_template_list) | **GET** /api/label/template/ | 
[**label_template_partial_update**](LabelApi.md#label_template_partial_update) | **PATCH** /api/label/template/{id}/ | 
[**label_template_retrieve**](LabelApi.md#label_template_retrieve) | **GET** /api/label/template/{id}/ | 
[**label_template_update**](LabelApi.md#label_template_update) | **PUT** /api/label/template/{id}/ | 



## label_print_create

> models::LabelPrint label_print_create(label_print)


POST action for printing labels.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**label_print** | [**LabelPrint**](LabelPrint.md) |  | [required] |

### Return type

[**models::LabelPrint**](LabelPrint.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## label_template_create

> models::LabelTemplate label_template_create(label_template)


API endpoint for viewing list of LabelTemplate objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**label_template** | [**LabelTemplate**](LabelTemplate.md) |  | [required] |

### Return type

[**models::LabelTemplate**](LabelTemplate.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## label_template_destroy

> label_template_destroy(id)


Detail API endpoint for label template model.

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


## label_template_list

> models::PaginatedLabelTemplateList label_template_list(limit, enabled, items, model_type, offset, search)


API endpoint for viewing list of LabelTemplate objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | **i32** | Number of results to return per page. | [required] |
**enabled** | Option<**bool**> |  |  |
**items** | Option<**String**> | Items |  |
**model_type** | Option<**String**> | Model Type  * `part` - Part * `build` - Build Order * `buildline` - Build Order Line Item * `company` - Company * `purchaseorder` - Purchase Order * `salesorder` - Sales Order * `salesordershipment` - Sales Order Shipment * `returnorder` - Return Order * `stockitem` - Stock Item * `stocklocation` - Stock Location |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |
**search** | Option<**String**> | A search term. Searched fields: description, name. |  |

### Return type

[**models::PaginatedLabelTemplateList**](PaginatedLabelTemplateList.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## label_template_partial_update

> models::LabelTemplate label_template_partial_update(id, patched_label_template)


Detail API endpoint for label template model.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**patched_label_template** | Option<[**PatchedLabelTemplate**](PatchedLabelTemplate.md)> |  |  |

### Return type

[**models::LabelTemplate**](LabelTemplate.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## label_template_retrieve

> models::LabelTemplate label_template_retrieve(id)


Detail API endpoint for label template model.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::LabelTemplate**](LabelTemplate.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## label_template_update

> models::LabelTemplate label_template_update(id, label_template)


Detail API endpoint for label template model.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**label_template** | [**LabelTemplate**](LabelTemplate.md) |  | [required] |

### Return type

[**models::LabelTemplate**](LabelTemplate.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

