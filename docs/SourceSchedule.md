# SourceSchedule

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**enabled** | **bool** | When `true`, the source is re-crawled based on the **frequency** field in this object. When `false` the source is not re-crawled; When `false` and connecting to Salesforce the source is crawled annually. | [optional] [default to true]
**time_zone** | **String** | The time zone to base source crawl times on. Possible values correspond to the IANA (Internet Assigned Numbers Authority) time zones list. | [optional] [default to America/New_York]
**frequency** | **String** | The crawl schedule in the specified **time_zone**.  -  `daily`: Runs every day between 00:00 and 06:00. -  `weekly`: Runs every week on Sunday between 00:00 and 06:00. -  `monthly`: Runs the on the first Sunday of every month between 00:00 and 06:00. | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


