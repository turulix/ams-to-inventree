# ScheduledTask

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pk** | **i32** |  | [readonly]
**name** | Option<**String**> |  | [optional]
**func** | **String** | e.g. module.tasks.function | 
**args** | Option<**String**> | e.g. 1, 2, 'John' | [optional]
**kwargs** | Option<**String**> | e.g. x=1, y=2, name='John' | [optional]
**schedule_type** | Option<[**models::ScheduleTypeEnum**](ScheduleTypeEnum.md)> |  | [optional]
**repeats** | Option<**i64**> | n = n times, -1 = forever | [optional]
**last_run** | **String** |  | 
**next_run** | Option<**String**> |  | [optional]
**success** | **bool** |  | 
**task** | Option<**String**> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


