
----------------------
-- Meta Information --
----------------------

-- number of parses (data points in time)
select count(*) from parses;

-- Time taken to parse codebase over time (assuming a continuous parsing without interruptions)
select (select min(created_on) from completed_parses) as first_parse,
(select max(created_on) from completed_parses) as last_parse,
age((select max(created_on) from completed_parses), (select min(created_on) from completed_parses)) as time_taken;

-- span of time of information
select (select min(time) from parses) as first_timepoint,
(select max(time) from parses) as last_timepoint,
age((select max(time) from parses), (select min(time) from parses)) as span_of_time;

-- percentage parses completion
select (select count(*) from completed_parses)::float / (select count(*) from git_tags) * 100;


--------------
-- Analysis --
--------------

-- language (lines, code, comments, files) over time
-- (selects distinct days and loses languages on same day but different timestamps)
select distinct cast (p.time as date), p.git_tag, sum(l.lines), sum(l.code), sum(l.comments)
from parses p
inner join languages l on l.parse_id = p.parse_id
group by p.parse_id
order by p.time asc;
