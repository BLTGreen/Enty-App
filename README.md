# Enty

Enty is a project using Rust and a Neo4j GraphDb to track a person's media consumption and surface insights such as: favourite genres, overlapping key word usage, preferred moods at different times of the year.

## Architectural Decisions

### GraphQL

I'm using GraphQL here to better access the data involved. I'm organising it into modular schemas to keep it modular.

### Determining the Data Domains

ERD Diagram TK

I have three super domains:
* Media
* Person
* Company

From there I can determine the links - i.e., Time Warner published Looney Tunes and Tegan and Sara's This Business of Art. 

I can also track multi-media artists - i.e., John Darnielle released the album All Hail West Texas and the book Wolf in White Van. Terry Pratchett wrote the Discworld series and also contributed to a mod for Oblivion. Sam Reich appears in Jake and Amir sketches, and is the CEO of Dropout which publishes Dimension 20.

From there I have subdomains around genre, type of media, associated keywords. 

#### Data Access Pattern

Data Access Pattern Doc TK
