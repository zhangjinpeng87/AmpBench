# AmpBench

This is a benchmark tool to measure the write/read/space amplification of SQL databases.

## Why
- Traditional SQL database benchmark tools focus on the performance of database, 
like latency and max QPS. Database deployed on cloud is more and more popular, 
and the resources include network, disk bandwidth are limited on the cloud. This
tool will let you know the amplification charactiristcs of your SQL database 
and indicate you to do deeper optimizations.

- Traditional benchmark tools like sysbench and tpcc, their table schema is simple 
and can't be modified by user. But the way of applications use database varies a 
lot. This tool can let users to define the table schema that is more similar with 
their real world workload. Users can define how many columns there are, what the 
columns type they are, how many secondary indexes there are, what the primary index
looks like, etc.
