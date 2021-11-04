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
bus-factor --language rust --project-count 50

project: 996.ICU user: 996icu percentage: 0.80
project: ripgrep user: BurntSushi percentage: 0.89
project: swc user: kdy1 percentage: 0.79
project: Rocket user: SergioBenitez percentage: 0.86
project: exa user: ogham percentage: 0.85
project: rustdesk user: rustdesk percentage: 0.85
project: sonic user: valeriansaliou percentage: 0.94
project: iced user: hecrj percentage: 0.88
project: delta user: dandavison percentage: 0.88
project: navi user: denisidoro percentage: 0.79
project: hyper user: seanmonstar percentage: 0.79
project: book user: carols10cents percentage: 0.76
project: xsv user: BurntSushi percentage: 0.92
project: py-spy user: benfred percentage: 0.81

## Using

`docker build --build-arg REPO_NAME="bus-factor" -t bus-factor:latest .`

`docker run --env-file Docker-test.env bus-factor:latest bus-factor --language <language> --project-count <project_count>`

