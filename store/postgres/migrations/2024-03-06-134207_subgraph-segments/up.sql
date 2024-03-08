-- Your SQL goes here
create table subgraphs.segments(
  id            serial primary key,
  deployment    int4 references subgraph_deployment(id) on delete cascade,
  start_block   int4 not null
  end_block     int4 not null
  current_block int4 null,
);
