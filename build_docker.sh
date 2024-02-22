#!/bin/bash

cd zsign && bash build_docker.sh
cd ..
docker build . -t sign