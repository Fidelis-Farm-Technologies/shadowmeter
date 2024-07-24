
echo "COPY (SELECT * FROM flow) TO 'flow.parquet' (FORMAT 'parquet')" | duckdb test.yaf.flow 

echo "COPY (SELECT * FROM flow) TO 'flow.csv' WITH (HEADER, DELIMITER '|') " |  duckdb test.yaf.flow 

echo "COPY (SELECT * FROM flow) TO 'flow.json';" |  duckdb test.yaf.flow 


