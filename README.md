# Snapshot

## Process Flow
- Running the application we take in the config for the dataset and first validate and ensure all of the 
    fields are properly configured. If config valid, continue, if not bail and let user know
- When config passes, check the dataset to ensure that the format is a supported file format, and that the features
    being asked to check are valid in the dataframe being passed. If that passes, then we ensure that any SQL expression that is passed as a transformation is valid and not malicious or malformed. Also ensure that any bounds that are passed are correct / are needed
- If all of these pass, then come up with a plan for how things are execute including: each feature being checked, the columns and bounds being checked, and the generated SQL that will be used
- Use generated plan to execute all checks that will happen, including executing SQL and the result and storing to datbase
- Should be some validate functionality that defines the plan and saves the result to JSON for the user to play around with and test. If they apply then we pull the json and run checks that way.

## Defined as Complete V1
When all of the above happens. We will also include:
- Sqlx to store the results of the check to a sqlite database, allowing for point in time correctness snapshots of data checks
- simple API to serve the data quality checks for the respective files available
- All components above are able to process the good and bad example files in the application
