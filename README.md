# crond
Cron task daemon, enables individuals to participate in the cron service network

## Archived!

This repo has been archived, because the croncat CLI will be made in js allowing users a browser based experience in addition to docker. This repo is for reference only.

## Setup

* 1. Clone `git clone git@github.com:Cron-Near/crond.git`
* 2. Change directory: `cd crond`
* 3. Start the daemon: `cargo run`

## For Cron Agents

If you have a reliable server, ready to run a docker instance with less than 0.1% downtime, you are elligible for earning revenue by executing cron tasks.

### TBD -- TC finish!

* docker pull cron-near/task-manager
* docker up -d
* near call 

NOTE:S
* docker needs to hold a small amount of NEAR to pay for signing txns
* docker needs to view/poll from the contract to check if new task
* docker needs to trigger the cron contract to trigger registered task contract
* docker initially registers with its node account id (the one making the payment), public key of node, beneficiary account id (for rewards upon success of task)

NOTE NOTE:
* Rewards: MVP will return immediately in the callback, could we do batching of rewards to take less fees overall?

## Documentation

* Run: `make docs`
  * OR: `make docs-view` to open locally in the browser

