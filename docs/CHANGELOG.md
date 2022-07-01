# 0.6.0..HEAD (2022-06-28)

- :sparkles: Separate PUT requests into two different requests. 
- :wrench: Add a configuration for gitmoji-changelog and a changelog. 

### Notes
    
- We need to be able to separate our update requests into two separate
  requests: One for data uris and one for files. This commit performs this
  separation.

- Using gitmoji-changelog is a little easier than trying to hand-generate the
  changlogs.
