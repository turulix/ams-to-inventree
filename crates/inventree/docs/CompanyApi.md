# \CompanyApi

All URIs are relative to *http://localhost:8000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**company_address_bulk_destroy**](CompanyApi.md#company_address_bulk_destroy) | **DELETE** /api/company/address/ | 
[**company_address_create**](CompanyApi.md#company_address_create) | **POST** /api/company/address/ | 
[**company_address_destroy**](CompanyApi.md#company_address_destroy) | **DELETE** /api/company/address/{id}/ | 
[**company_address_list**](CompanyApi.md#company_address_list) | **GET** /api/company/address/ | 
[**company_address_partial_update**](CompanyApi.md#company_address_partial_update) | **PATCH** /api/company/address/{id}/ | 
[**company_address_retrieve**](CompanyApi.md#company_address_retrieve) | **GET** /api/company/address/{id}/ | 
[**company_address_update**](CompanyApi.md#company_address_update) | **PUT** /api/company/address/{id}/ | 
[**company_contact_bulk_destroy**](CompanyApi.md#company_contact_bulk_destroy) | **DELETE** /api/company/contact/ | 
[**company_contact_create**](CompanyApi.md#company_contact_create) | **POST** /api/company/contact/ | 
[**company_contact_destroy**](CompanyApi.md#company_contact_destroy) | **DELETE** /api/company/contact/{id}/ | 
[**company_contact_list**](CompanyApi.md#company_contact_list) | **GET** /api/company/contact/ | 
[**company_contact_partial_update**](CompanyApi.md#company_contact_partial_update) | **PATCH** /api/company/contact/{id}/ | 
[**company_contact_retrieve**](CompanyApi.md#company_contact_retrieve) | **GET** /api/company/contact/{id}/ | 
[**company_contact_update**](CompanyApi.md#company_contact_update) | **PUT** /api/company/contact/{id}/ | 
[**company_create**](CompanyApi.md#company_create) | **POST** /api/company/ | 
[**company_destroy**](CompanyApi.md#company_destroy) | **DELETE** /api/company/{id}/ | 
[**company_list**](CompanyApi.md#company_list) | **GET** /api/company/ | 
[**company_part_bulk_destroy**](CompanyApi.md#company_part_bulk_destroy) | **DELETE** /api/company/part/ | 
[**company_part_create**](CompanyApi.md#company_part_create) | **POST** /api/company/part/ | 
[**company_part_destroy**](CompanyApi.md#company_part_destroy) | **DELETE** /api/company/part/{id}/ | 
[**company_part_list**](CompanyApi.md#company_part_list) | **GET** /api/company/part/ | 
[**company_part_manufacturer_bulk_destroy**](CompanyApi.md#company_part_manufacturer_bulk_destroy) | **DELETE** /api/company/part/manufacturer/ | 
[**company_part_manufacturer_create**](CompanyApi.md#company_part_manufacturer_create) | **POST** /api/company/part/manufacturer/ | 
[**company_part_manufacturer_destroy**](CompanyApi.md#company_part_manufacturer_destroy) | **DELETE** /api/company/part/manufacturer/{id}/ | 
[**company_part_manufacturer_list**](CompanyApi.md#company_part_manufacturer_list) | **GET** /api/company/part/manufacturer/ | 
[**company_part_manufacturer_partial_update**](CompanyApi.md#company_part_manufacturer_partial_update) | **PATCH** /api/company/part/manufacturer/{id}/ | 
[**company_part_manufacturer_retrieve**](CompanyApi.md#company_part_manufacturer_retrieve) | **GET** /api/company/part/manufacturer/{id}/ | 
[**company_part_manufacturer_update**](CompanyApi.md#company_part_manufacturer_update) | **PUT** /api/company/part/manufacturer/{id}/ | 
[**company_part_partial_update**](CompanyApi.md#company_part_partial_update) | **PATCH** /api/company/part/{id}/ | 
[**company_part_retrieve**](CompanyApi.md#company_part_retrieve) | **GET** /api/company/part/{id}/ | 
[**company_part_update**](CompanyApi.md#company_part_update) | **PUT** /api/company/part/{id}/ | 
[**company_partial_update**](CompanyApi.md#company_partial_update) | **PATCH** /api/company/{id}/ | 
[**company_price_break_create**](CompanyApi.md#company_price_break_create) | **POST** /api/company/price-break/ | 
[**company_price_break_destroy**](CompanyApi.md#company_price_break_destroy) | **DELETE** /api/company/price-break/{id}/ | 
[**company_price_break_list**](CompanyApi.md#company_price_break_list) | **GET** /api/company/price-break/ | 
[**company_price_break_partial_update**](CompanyApi.md#company_price_break_partial_update) | **PATCH** /api/company/price-break/{id}/ | 
[**company_price_break_retrieve**](CompanyApi.md#company_price_break_retrieve) | **GET** /api/company/price-break/{id}/ | 
[**company_price_break_update**](CompanyApi.md#company_price_break_update) | **PUT** /api/company/price-break/{id}/ | 
[**company_retrieve**](CompanyApi.md#company_retrieve) | **GET** /api/company/{id}/ | 
[**company_update**](CompanyApi.md#company_update) | **PUT** /api/company/{id}/ | 



## company_address_bulk_destroy

> company_address_bulk_destroy(bulk_request)


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


## company_address_create

> models::Address company_address_create(address)


API endpoint for list view of Address model.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | [**Address**](Address.md) |  | [required] |

### Return type

[**models::Address**](Address.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## company_address_destroy

> company_address_destroy(id)


API endpoint for a single Address object.

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


## company_address_list

> models::PaginatedAddressList company_address_list(limit, company, offset, ordering, search)


Override the GET method to determine export options.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | **i32** | Number of results to return per page. | [required] |
**company** | Option<**i32**> |  |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**search** | Option<**String**> | A search term. |  |

### Return type

[**models::PaginatedAddressList**](PaginatedAddressList.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## company_address_partial_update

> models::Address company_address_partial_update(id, patched_address)


API endpoint for a single Address object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**patched_address** | Option<[**PatchedAddress**](PatchedAddress.md)> |  |  |

### Return type

[**models::Address**](Address.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## company_address_retrieve

> models::Address company_address_retrieve(id)


API endpoint for a single Address object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::Address**](Address.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## company_address_update

> models::Address company_address_update(id, address)


API endpoint for a single Address object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**address** | [**Address**](Address.md) |  | [required] |

### Return type

[**models::Address**](Address.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## company_contact_bulk_destroy

> company_contact_bulk_destroy(bulk_request)


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


## company_contact_create

> models::Contact company_contact_create(contact)


API endpoint for list view of Company model.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contact** | [**Contact**](Contact.md) |  | [required] |

### Return type

[**models::Contact**](Contact.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## company_contact_destroy

> company_contact_destroy(id)


Detail endpoint for Company model.

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


## company_contact_list

> models::PaginatedContactList company_contact_list(limit, company, offset, ordering, search)


Override the GET method to determine export options.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | **i32** | Number of results to return per page. | [required] |
**company** | Option<**i32**> |  |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**search** | Option<**String**> | A search term. Searched fields: company__name, name. |  |

### Return type

[**models::PaginatedContactList**](PaginatedContactList.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## company_contact_partial_update

> models::Contact company_contact_partial_update(id, patched_contact)


Detail endpoint for Company model.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**patched_contact** | Option<[**PatchedContact**](PatchedContact.md)> |  |  |

### Return type

[**models::Contact**](Contact.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## company_contact_retrieve

> models::Contact company_contact_retrieve(id)


Detail endpoint for Company model.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::Contact**](Contact.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## company_contact_update

> models::Contact company_contact_update(id, contact)


Detail endpoint for Company model.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**contact** | [**Contact**](Contact.md) |  | [required] |

### Return type

[**models::Contact**](Contact.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## company_create

> models::Company company_create(company)


API endpoint for accessing a list of Company objects.  Provides two methods:  - GET: Return list of objects - POST: Create a new Company object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company** | [**Company**](Company.md) |  | [required] |

### Return type

[**models::Company**](Company.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## company_destroy

> company_destroy(id)


API endpoint for detail of a single Company object.

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


## company_list

> models::PaginatedCompanyList company_list(limit, active, is_customer, is_manufacturer, is_supplier, name, offset, ordering, search)


Override the GET method to determine export options.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | **i32** | Number of results to return per page. | [required] |
**active** | Option<**bool**> |  |  |
**is_customer** | Option<**bool**> |  |  |
**is_manufacturer** | Option<**bool**> |  |  |
**is_supplier** | Option<**bool**> |  |  |
**name** | Option<**String**> |  |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**search** | Option<**String**> | A search term. Searched fields: description, name, tax_id, website. |  |

### Return type

[**models::PaginatedCompanyList**](PaginatedCompanyList.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## company_part_bulk_destroy

> company_part_bulk_destroy(bulk_request)


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


## company_part_create

> models::SupplierPart company_part_create(supplier_part)


API endpoint for list view of SupplierPart object.  - GET: Return list of SupplierPart objects - POST: Create a new SupplierPart object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**supplier_part** | [**SupplierPart**](SupplierPart.md) |  | [required] |

### Return type

[**models::SupplierPart**](SupplierPart.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## company_part_destroy

> company_part_destroy(id)


API endpoint for detail view of SupplierPart object.  - GET: Retrieve detail view - PATCH: Update object - DELETE: Delete object

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


## company_part_list

> models::PaginatedSupplierPartList company_part_list(limit, mpn, sku, active, company, has_stock, manufacturer, manufacturer_detail, manufacturer_part, manufacturer_part_detail, offset, ordering, part, part_active, part_detail, pretty, search, supplier, supplier_active, supplier_detail, tags__name, tags__slug)


Override the GET method to determine export options.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | **i32** | Number of results to return per page. | [required] |
**mpn** | Option<**String**> | Manufacturer Part Number |  |
**sku** | Option<**String**> |  |  |
**active** | Option<**bool**> | Supplier Part is Active |  |
**company** | Option<**i32**> | Company |  |
**has_stock** | Option<**bool**> | Has Stock |  |
**manufacturer** | Option<**i32**> | Manufacturer |  |
**manufacturer_detail** | Option<**bool**> | Include detailed information about the Manufacturer in the response |  |[default to false]
**manufacturer_part** | Option<**i32**> |  |  |
**manufacturer_part_detail** | Option<**bool**> | Include detailed information about the linked ManufacturerPart in the response |  |[default to false]
**offset** | Option<**i32**> | The initial index from which to return the results. |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**part** | Option<**i32**> |  |  |
**part_active** | Option<**bool**> | Internal Part is Active |  |
**part_detail** | Option<**bool**> | Include detailed information about the linked Part in the response |  |[default to false]
**pretty** | Option<**bool**> | Format the output with a more readable (pretty) name |  |[default to false]
**search** | Option<**String**> | A search term. Searched fields: SKU, description, manufacturer_part__MPN, manufacturer_part__manufacturer__name, part__IPN, part__description, part__keywords, part__name, supplier__name, tags__name, tags__slug. |  |
**supplier** | Option<**i32**> |  |  |
**supplier_active** | Option<**bool**> | Supplier is Active |  |
**supplier_detail** | Option<**bool**> | Include detailed information about the Supplier in the response |  |[default to false]
**tags__name** | Option<**String**> |  |  |
**tags__slug** | Option<**String**> |  |  |

### Return type

[**models::PaginatedSupplierPartList**](PaginatedSupplierPartList.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## company_part_manufacturer_bulk_destroy

> company_part_manufacturer_bulk_destroy(bulk_request)


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


## company_part_manufacturer_create

> models::ManufacturerPart company_part_manufacturer_create(manufacturer_part)


API endpoint for list view of ManufacturerPart object.  - GET: Return list of ManufacturerPart objects - POST: Create a new ManufacturerPart object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**manufacturer_part** | [**ManufacturerPart**](ManufacturerPart.md) |  | [required] |

### Return type

[**models::ManufacturerPart**](ManufacturerPart.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## company_part_manufacturer_destroy

> company_part_manufacturer_destroy(id)


API endpoint for detail view of ManufacturerPart object.  - GET: Retrieve detail view - PATCH: Update object - DELETE: Delete object

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


## company_part_manufacturer_list

> models::PaginatedManufacturerPartList company_part_manufacturer_list(limit, mpn, manufacturer, manufacturer_active, manufacturer_detail, offset, ordering, part, part_active, part_detail, pretty, search, tags__name, tags__slug)


API endpoint for list view of ManufacturerPart object.  - GET: Return list of ManufacturerPart objects - POST: Create a new ManufacturerPart object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | **i32** | Number of results to return per page. | [required] |
**mpn** | Option<**String**> |  |  |
**manufacturer** | Option<**i32**> |  |  |
**manufacturer_active** | Option<**bool**> | Manufacturer is Active |  |
**manufacturer_detail** | Option<**bool**> | Include detailed information about the Manufacturer in the response |  |[default to false]
**offset** | Option<**i32**> | The initial index from which to return the results. |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**part** | Option<**i32**> |  |  |
**part_active** | Option<**bool**> | Part is Active |  |
**part_detail** | Option<**bool**> | Include detailed information about the linked Part in the response |  |[default to false]
**pretty** | Option<**bool**> | Format the output with a more readable (pretty) name |  |[default to false]
**search** | Option<**String**> | A search term. Searched fields: MPN, description, manufacturer__name, part__IPN, part__description, part__name, tags__name, tags__slug. |  |
**tags__name** | Option<**String**> |  |  |
**tags__slug** | Option<**String**> |  |  |

### Return type

[**models::PaginatedManufacturerPartList**](PaginatedManufacturerPartList.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## company_part_manufacturer_partial_update

> models::ManufacturerPart company_part_manufacturer_partial_update(id, patched_manufacturer_part)


API endpoint for detail view of ManufacturerPart object.  - GET: Retrieve detail view - PATCH: Update object - DELETE: Delete object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**patched_manufacturer_part** | Option<[**PatchedManufacturerPart**](PatchedManufacturerPart.md)> |  |  |

### Return type

[**models::ManufacturerPart**](ManufacturerPart.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## company_part_manufacturer_retrieve

> models::ManufacturerPart company_part_manufacturer_retrieve(id)


API endpoint for detail view of ManufacturerPart object.  - GET: Retrieve detail view - PATCH: Update object - DELETE: Delete object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::ManufacturerPart**](ManufacturerPart.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## company_part_manufacturer_update

> models::ManufacturerPart company_part_manufacturer_update(id, manufacturer_part)


API endpoint for detail view of ManufacturerPart object.  - GET: Retrieve detail view - PATCH: Update object - DELETE: Delete object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**manufacturer_part** | [**ManufacturerPart**](ManufacturerPart.md) |  | [required] |

### Return type

[**models::ManufacturerPart**](ManufacturerPart.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## company_part_partial_update

> models::SupplierPart company_part_partial_update(id, patched_supplier_part)


API endpoint for detail view of SupplierPart object.  - GET: Retrieve detail view - PATCH: Update object - DELETE: Delete object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**patched_supplier_part** | Option<[**PatchedSupplierPart**](PatchedSupplierPart.md)> |  |  |

### Return type

[**models::SupplierPart**](SupplierPart.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## company_part_retrieve

> models::SupplierPart company_part_retrieve(id, manufacturer_detail, manufacturer_part_detail, part_detail, pretty, supplier_detail)


API endpoint for detail view of SupplierPart object.  - GET: Retrieve detail view - PATCH: Update object - DELETE: Delete object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**manufacturer_detail** | Option<**bool**> | Include detailed information about the Manufacturer in the response |  |[default to false]
**manufacturer_part_detail** | Option<**bool**> | Include detailed information about the linked ManufacturerPart in the response |  |[default to false]
**part_detail** | Option<**bool**> | Include detailed information about the linked Part in the response |  |[default to false]
**pretty** | Option<**bool**> | Format the output with a more readable (pretty) name |  |[default to false]
**supplier_detail** | Option<**bool**> | Include detailed information about the Supplier in the response |  |[default to false]

### Return type

[**models::SupplierPart**](SupplierPart.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## company_part_update

> models::SupplierPart company_part_update(id, supplier_part)


API endpoint for detail view of SupplierPart object.  - GET: Retrieve detail view - PATCH: Update object - DELETE: Delete object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**supplier_part** | [**SupplierPart**](SupplierPart.md) |  | [required] |

### Return type

[**models::SupplierPart**](SupplierPart.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## company_partial_update

> models::Company company_partial_update(id, patched_company)


API endpoint for detail of a single Company object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**patched_company** | Option<[**PatchedCompany**](PatchedCompany.md)> |  |  |

### Return type

[**models::Company**](Company.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## company_price_break_create

> models::SupplierPriceBreak company_price_break_create(supplier_price_break)


API endpoint for list view of SupplierPriceBreak object.  - GET: Retrieve list of SupplierPriceBreak objects - POST: Create a new SupplierPriceBreak object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**supplier_price_break** | [**SupplierPriceBreak**](SupplierPriceBreak.md) |  | [required] |

### Return type

[**models::SupplierPriceBreak**](SupplierPriceBreak.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## company_price_break_destroy

> company_price_break_destroy(id)


Detail endpoint for SupplierPriceBreak object.

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


## company_price_break_list

> models::PaginatedSupplierPriceBreakList company_price_break_list(limit, base_part, offset, ordering, part, part_detail, quantity, search, supplier, supplier_detail)


Override the GET method to determine export options.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | **i32** | Number of results to return per page. | [required] |
**base_part** | Option<**i32**> | Base Part |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**part** | Option<**i32**> |  |  |
**part_detail** | Option<**bool**> | Include detailed information about the linked Part in the response |  |[default to false]
**quantity** | Option<**f64**> |  |  |
**search** | Option<**String**> | A search term. Searched fields: part__SKU, part__supplier__name. |  |
**supplier** | Option<**i32**> | Supplier |  |
**supplier_detail** | Option<**bool**> | Include detailed information about the Supplier in the response |  |[default to false]

### Return type

[**models::PaginatedSupplierPriceBreakList**](PaginatedSupplierPriceBreakList.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## company_price_break_partial_update

> models::SupplierPriceBreak company_price_break_partial_update(id, patched_supplier_price_break)


Detail endpoint for SupplierPriceBreak object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**patched_supplier_price_break** | Option<[**PatchedSupplierPriceBreak**](PatchedSupplierPriceBreak.md)> |  |  |

### Return type

[**models::SupplierPriceBreak**](SupplierPriceBreak.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## company_price_break_retrieve

> models::SupplierPriceBreak company_price_break_retrieve(id)


Detail endpoint for SupplierPriceBreak object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::SupplierPriceBreak**](SupplierPriceBreak.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## company_price_break_update

> models::SupplierPriceBreak company_price_break_update(id, supplier_price_break)


Detail endpoint for SupplierPriceBreak object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**supplier_price_break** | [**SupplierPriceBreak**](SupplierPriceBreak.md) |  | [required] |

### Return type

[**models::SupplierPriceBreak**](SupplierPriceBreak.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## company_retrieve

> models::Company company_retrieve(id)


API endpoint for detail of a single Company object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::Company**](Company.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## company_update

> models::Company company_update(id, company)


API endpoint for detail of a single Company object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**company** | [**Company**](Company.md) |  | [required] |

### Return type

[**models::Company**](Company.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

