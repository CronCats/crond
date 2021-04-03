# Cron Task Runners

Task execution will be handled by external NEAR blockchain participant nodes that run a Cron code package. 

### Register as a task runner

As a task runner, I want to provide Cron with a reliable ongoing triggering mechanism by registering my server as an official Cron agent. I expect to create a special wallet my server can utilize to sign transactions for the NEAR blockchain. I expect to keep my server 100% accessible to the internet and to NEAR blockchain. I expect to register my server as a participant in the Cron service by submitting my server wallet account ID, account public key, reward account id and possibly a graffiti tag for friendly runner leaderboards. I expect the registration process to take some small window of block times, and potentially need to stake some NEAR tokens to help secure my place as a Cron agent. I also expect to incur fees or total stake loss upon acting maliciously toward the Cron protocol.

### Unregister as a task runner

As a task runner, I want to remove my agent from running any further cron tasks. I expect to cancel any upcoming tasks that may have been pre-assigned, but do not expect any tasks that are pending / submitted to be cancelled. For a successful unregister, I expect to withdraw any / all rewards accrued by my agent for running tasks. I expect to not be able to withdraw some or all rewards in the event my agent was deemed malicious. I expect that I can re-register at a future date if my account is deemed acceptable again in the future.

### Viewing available tasks

As a task runner, I want to view all tasks that my agent can run. I expect to be able to view this list or paginate through a list of available tasks to make sure my agent has tasks available to run. In most cases this list will be how my agent knows if it should be signing a transaction or maintaining an idle but active status.

### Executing an available task or task batch

As a task runner, I want to claim a task from the Cron queue. To claim a task, I expect my agent to autonomously request an item from Cron that I can sign as a transaction and submit to the network. Upon successful execution, I expect my agent to become available to process another task, repeating this cycle until there are no further tasks or my agent is un-registering.

### Accrueing & claiming rewards

As a task runner, I want to profit for running tasks by accruing small fees per task. My rewards will be automatically paid out directly to my configured beneficiary account ID at time of execution or in batches. To participate in being an agent, I understand that I may be required to lock an amount of NEAR within Cron as a security for upholding my agents goodwill and reputation.
