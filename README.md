# bus-factor 
Program finds popular GitHub projects with a bus factor of 1.

**Input**: programming language name, number of projects to consider.

**Output**: list of tuples containing the project's name, top contributor's name and their
contribution percentage.

With given programming language name (` language `) and a project count ( `project_count` ),
program fetch the first `project_count` most popular projects (sorted by the
number of GitHub stars) from the given `language`.
Then, for each project, its contributor statistics are inspected.
Project's bus factor is 1 if its most active developer's contributions
account for 75% or more of the total contributions count from the top 25 most active
developers.

## What does bus factor mean?
Bus factor is a measurement which attempts to estimate the number of key persons a
project would need to lose in order for it to become stalled due to lack of expertise. It is
commonly used in the context of software development.

For example, if a given project is developed by a single person, then the project's bus
factor is equal to 1 (it's likely for the project to become unmaintained if the main
contributor suddenly stops working on it).

## Example

## Using

`docker build --build-arg REPO_NAME="bus-factor" -t bus-factor:latest .`

`docker run bus-factor:latest`

