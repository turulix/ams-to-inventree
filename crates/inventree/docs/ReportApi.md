# \ReportApi

All URIs are relative to *http://localhost:8000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**report_asset_create**](ReportApi.md#report_asset_create) | **POST** /api/report/asset/ | 
[**report_asset_destroy**](ReportApi.md#report_asset_destroy) | **DELETE** /api/report/asset/{id}/ | 
[**report_asset_list**](ReportApi.md#report_asset_list) | **GET** /api/report/asset/ | 
[**report_asset_partial_update**](ReportApi.md#report_asset_partial_update) | **PATCH** /api/report/asset/{id}/ | 
[**report_asset_retrieve**](ReportApi.md#report_asset_retrieve) | **GET** /api/report/asset/{id}/ | 
[**report_asset_update**](ReportApi.md#report_asset_update) | **PUT** /api/report/asset/{id}/ | 
[**report_print_create**](ReportApi.md#report_print_create) | **POST** /api/report/print/ | 
[**report_snippet_create**](ReportApi.md#report_snippet_create) | **POST** /api/report/snippet/ | 
[**report_snippet_destroy**](ReportApi.md#report_snippet_destroy) | **DELETE** /api/report/snippet/{id}/ | 
[**report_snippet_list**](ReportApi.md#report_snippet_list) | **GET** /api/report/snippet/ | 
[**report_snippet_partial_update**](ReportApi.md#report_snippet_partial_update) | **PATCH** /api/report/snippet/{id}/ | 
[**report_snippet_retrieve**](ReportApi.md#report_snippet_retrieve) | **GET** /api/report/snippet/{id}/ | 
[**report_snippet_update**](ReportApi.md#report_snippet_update) | **PUT** /api/report/snippet/{id}/ | 
[**report_template_create**](ReportApi.md#report_template_create) | **POST** /api/report/template/ | 
[**report_template_destroy**](ReportApi.md#report_template_destroy) | **DELETE** /api/report/template/{id}/ | 
[**report_template_list**](ReportApi.md#report_template_list) | **GET** /api/report/template/ | 
[**report_template_partial_update**](ReportApi.md#report_template_partial_update) | **PATCH** /api/report/template/{id}/ | 
[**report_template_retrieve**](ReportApi.md#report_template_retrieve) | **GET** /api/report/template/{id}/ | 
[**report_template_update**](ReportApi.md#report_template_update) | **PUT** /api/report/template/{id}/ | 



## report_asset_create

> models::ReportAsset report_asset_create(report_asset)


API endpoint for listing ReportAsset objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**report_asset** | [**ReportAsset**](ReportAsset.md) |  | [required] |

### Return type

[**models::ReportAsset**](ReportAsset.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## report_asset_destroy

> report_asset_destroy(id)


API endpoint for a single ReportAsset object.

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


## report_asset_list

> models::PaginatedReportAssetList report_asset_list(limit, offset)


API endpoint for listing ReportAsset objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | **i32** | Number of results to return per page. | [required] |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |

### Return type

[**models::PaginatedReportAssetList**](PaginatedReportAssetList.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## report_asset_partial_update

> models::ReportAsset report_asset_partial_update(id, patched_report_asset)


API endpoint for a single ReportAsset object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**patched_report_asset** | Option<[**PatchedReportAsset**](PatchedReportAsset.md)> |  |  |

### Return type

[**models::ReportAsset**](ReportAsset.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## report_asset_retrieve

> models::ReportAsset report_asset_retrieve(id)


API endpoint for a single ReportAsset object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::ReportAsset**](ReportAsset.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## report_asset_update

> models::ReportAsset report_asset_update(id, report_asset)


API endpoint for a single ReportAsset object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**report_asset** | [**ReportAsset**](ReportAsset.md) |  | [required] |

### Return type

[**models::ReportAsset**](ReportAsset.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## report_print_create

> models::ReportPrint report_print_create(report_print)


POST action for printing a report.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**report_print** | [**ReportPrint**](ReportPrint.md) |  | [required] |

### Return type

[**models::ReportPrint**](ReportPrint.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## report_snippet_create

> models::ReportSnippet report_snippet_create(report_snippet)


API endpoint for listing ReportSnippet objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**report_snippet** | [**ReportSnippet**](ReportSnippet.md) |  | [required] |

### Return type

[**models::ReportSnippet**](ReportSnippet.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## report_snippet_destroy

> report_snippet_destroy(id)


API endpoint for a single ReportSnippet object.

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


## report_snippet_list

> models::PaginatedReportSnippetList report_snippet_list(limit, offset)


API endpoint for listing ReportSnippet objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | **i32** | Number of results to return per page. | [required] |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |

### Return type

[**models::PaginatedReportSnippetList**](PaginatedReportSnippetList.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## report_snippet_partial_update

> models::ReportSnippet report_snippet_partial_update(id, patched_report_snippet)


API endpoint for a single ReportSnippet object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**patched_report_snippet** | Option<[**PatchedReportSnippet**](PatchedReportSnippet.md)> |  |  |

### Return type

[**models::ReportSnippet**](ReportSnippet.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## report_snippet_retrieve

> models::ReportSnippet report_snippet_retrieve(id)


API endpoint for a single ReportSnippet object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::ReportSnippet**](ReportSnippet.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## report_snippet_update

> models::ReportSnippet report_snippet_update(id, report_snippet)


API endpoint for a single ReportSnippet object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**report_snippet** | [**ReportSnippet**](ReportSnippet.md) |  | [required] |

### Return type

[**models::ReportSnippet**](ReportSnippet.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## report_template_create

> models::ReportTemplate report_template_create(report_template)


API endpoint for viewing list of ReportTemplate objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**report_template** | [**ReportTemplate**](ReportTemplate.md) |  | [required] |

### Return type

[**models::ReportTemplate**](ReportTemplate.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## report_template_destroy

> report_template_destroy(id)


Detail API endpoint for report template model.

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


## report_template_list

> models::PaginatedReportTemplateList report_template_list(limit, enabled, items, landscape, model_type, offset, search)


API endpoint for viewing list of ReportTemplate objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | **i32** | Number of results to return per page. | [required] |
**enabled** | Option<**bool**> |  |  |
**items** | Option<**String**> | Items |  |
**landscape** | Option<**bool**> |  |  |
**model_type** | Option<**String**> | Model Type  * `part` - Part * `build` - Build Order * `buildline` - Build Order Line Item * `company` - Company * `purchaseorder` - Purchase Order * `salesorder` - Sales Order * `salesordershipment` - Sales Order Shipment * `returnorder` - Return Order * `stockitem` - Stock Item * `stocklocation` - Stock Location |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |
**search** | Option<**String**> | A search term. Searched fields: description, name. |  |

### Return type

[**models::PaginatedReportTemplateList**](PaginatedReportTemplateList.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## report_template_partial_update

> models::ReportTemplate report_template_partial_update(id, patched_report_template)


Detail API endpoint for report template model.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**patched_report_template** | Option<[**PatchedReportTemplate**](PatchedReportTemplate.md)> |  |  |

### Return type

[**models::ReportTemplate**](ReportTemplate.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## report_template_retrieve

> models::ReportTemplate report_template_retrieve(id)


Detail API endpoint for report template model.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::ReportTemplate**](ReportTemplate.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## report_template_update

> models::ReportTemplate report_template_update(id, report_template)


Detail API endpoint for report template model.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**report_template** | [**ReportTemplate**](ReportTemplate.md) |  | [required] |

### Return type

[**models::ReportTemplate**](ReportTemplate.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

