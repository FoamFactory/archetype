#!/bin/bash

if diesel migration pending > /dev/null 2>&1; then
    archetype
else
    diesel migration run && archetype
fi
