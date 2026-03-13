# UserCreate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pk** | **i32** |  | [readonly]
**username** | **String** | Username | 
**first_name** | **String** | First name of the user | 
**last_name** | **String** | Last name of the user | 
**email** | **String** | Email address of the user | 
**groups** | [**Vec<models::Group>**](Group.md) |  | [readonly]
**group_ids** | Option<**Vec<i32>**> |  | [optional]
**is_staff** | Option<**bool**> | Does this user have staff permissions | [optional]
**is_superuser** | Option<**bool**> | Is this user a superuser | [optional]
**is_active** | Option<**bool**> | Is this user account active | [optional]
**profile** | [**models::BriefUserProfile**](BriefUserProfile.md) |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


