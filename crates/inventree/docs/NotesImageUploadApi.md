# \NotesImageUploadApi

All URIs are relative to *http://localhost:8000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**notes_image_upload_create**](NotesImageUploadApi.md#notes_image_upload_create) | **POST** /api/notes-image-upload/ | 
[**notes_image_upload_list**](NotesImageUploadApi.md#notes_image_upload_list) | **GET** /api/notes-image-upload/ | 



## notes_image_upload_create

> models::NotesImage notes_image_upload_create(notes_image)


List view for all notes images.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**notes_image** | [**NotesImage**](NotesImage.md) |  | [required] |

### Return type

[**models::NotesImage**](NotesImage.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## notes_image_upload_list

> models::PaginatedNotesImageList notes_image_upload_list(limit, offset, ordering, search)


List view for all notes images.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | **i32** | Number of results to return per page. | [required] |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**search** | Option<**String**> | A search term. Searched fields: model_id, model_type, user. |  |

### Return type

[**models::PaginatedNotesImageList**](PaginatedNotesImageList.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

