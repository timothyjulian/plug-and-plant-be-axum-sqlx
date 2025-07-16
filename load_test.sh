#!/bin/bash


for batch in {1..1}; do
  echo "Batch $batch"
  start=$(gdate +%s%3N)


  for i in {1..10}; do
    curl --silent --location --request GET 'localhost:3000/' \
    --header 'Content-Type: application/json' \
    --data '{
        "hi":"test"
    }' &
  done

  wait
  end=$(gdate +%s%3N)
  duration=$((end - start))
  echo "Batch $batch completed in ${duration} ms"
  
  sleep 0.5
done
