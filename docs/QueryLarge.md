# QueryLarge

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**filter** | **String** | A cacheable query that excludes documents that don't mention the query content. Filter searches are better for metadata-type searches and for assessing the concepts in the data set. | [optional] 
**query** | **String** | A query search returns all documents in your data set with full enrichments and full text, but with the most relevant documents listed first. Use a query search when you want to find the most relevant search results. You cannot use **natural_language_query** and **query** at the same time. | [optional] 
**natural_language_query** | **String** | A natural language query that returns relevant documents by utilizing training data and natural language understanding. You cannot use **natural_language_query** and **query** at the same time. | [optional] 
**passages** | **bool** | A passages query that returns the most relevant passages from the results. | [optional] 
**aggregation** | **String** | An aggregation search that returns an exact answer by combining query search with filters. Useful for applications to build lists, tables, and time series. For a full list of possible aggregations, see the Query reference. | [optional] 
**count** | **i32** | Number of results to return. | [optional] 
**return_** | **String** | A comma-separated list of the portion of the document hierarchy to return. | [optional] 
**offset** | **i32** | The number of query results to skip at the beginning. For example, if the total number of results that are returned is 10 and the offset is 8, it returns the last two results. | [optional] 
**sort** | **String** | A comma-separated list of fields in the document to sort on. You can optionally specify a sort direction by prefixing the field with `-` for descending or `+` for ascending. Ascending is the default sort direction if no prefix is specified. This parameter cannot be used in the same query as the **bias** parameter. | [optional] 
**highlight** | **bool** | When true, a highlight field is returned for each result which contains the fields which match the query with `<em></em>` tags around the matching query terms. | [optional] [default to false]
**passages_fields** | **String** | A comma-separated list of fields that passages are drawn from. If this parameter not specified, then all top-level fields are included. | [optional] 
**passages_count** | **i32** | The maximum number of passages to return. The search returns fewer passages if the requested total is not found. The default is `10`. The maximum is `100`. | [optional] 
**passages_characters** | **i32** | The approximate number of characters that any one passage will have. | [optional] 
**deduplicate** | **bool** | When `true` and used with a Watson Discovery News collection, duplicate results (based on the contents of the **title** field) are removed. Duplicate comparison is limited to the current query only; **offset** is not considered. This parameter is currently Beta functionality. | [optional] [default to false]
**deduplicate_field** | **String** | When specified, duplicate results based on the field specified are removed from the returned results. Duplicate comparison is limited to the current query only, **offset** is not considered. This parameter is currently Beta functionality. | [optional] 
**collection_ids** | **String** | A comma-separated list of collection IDs to be queried against. Required when querying multiple collections, invalid when performing a single collection query. | [optional] 
**similar** | **bool** |  When `true`, results are returned based on their similarity to the document IDs specified in the **similar.document_ids** parameter. | [optional] [default to false]
**similar_document_ids** | **String** | A comma-separated list of document IDs to find similar documents.  **Tip:** Include the **natural_language_query** parameter to expand the scope of the document similarity search with the natural language query. Other query parameters, such as **filter** and **query**, are subsequently applied and reduce the scope. | [optional] 
**similar_fields** | **String** | A comma-separated list of field names that are used as a basis for comparison to identify similar documents. If not specified, the entire document is used for comparison. | [optional] 
**bias** | **String** | Field which the returned results will be biased against. The specified field must be either a **date** or **number** format. When a **date** type field is specified returned results are biased towards field values closer to the current date. When a **number** type field is specified, returned results are biased towards higher field values. This parameter cannot be used in the same query as the **sort** parameter. | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


