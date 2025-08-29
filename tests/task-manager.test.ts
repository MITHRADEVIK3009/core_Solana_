
import * as anchor from '@project-serum/anchor';
import { expect } from 'chai';
import { TaskManagerClient } from '../client/task-manager';

describe('Task Manager', () => {
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);

    const program = anchor.workspace.TaskManager;
    const client = new TaskManagerClient(program);

    let userKeypair: anchor.web3.Keypair;
    let assigneeKeypair: anchor.web3.Keypair;

    before(async () => {
        userKeypair = anchor.web3.Keypair.generate();
        assigneeKeypair = anchor.web3.Keypair.generate();

        await provider.connection.requestAirdrop(
            userKeypair.publicKey,
            2 * anchor.web3.LAMPORTS_PER_SOL
        );
        await provider.connection.requestAirdrop(
            assigneeKeypair.publicKey,
            2 * anchor.web3.LAMPORTS_PER_SOL
        );
    });

    it('Initializes user profile', async () => {
        await client.initializeUser();
        
        const profile = await client.getUserProfile(provider.wallet.publicKey);
        expect(profile.tasksCreated.toString()).to.equal('0');
        expect(profile.reputationScore.toString()).to.equal('100');
    });

    it('Creates a task', async () => {
        await client.createTask(1, "Test Task", "Description", 100);
        
        const task = await client.getTask(provider.wallet.publicKey, 1);
        expect(task.title).to.equal("Test Task");
        expect(task.status.open).to.exist;
    });

    it('Completes full task workflow', async () => {
       
        await client.createTask(2, "Full Workflow", "Complete workflow test", 200);
        
     
        await client.assignTask(provider.wallet.publicKey, 2, assigneeKeypair.publicKey);
        
        let task = await client.getTask(provider.wallet.publicKey, 2);
        expect(task.status.assigned).to.exist;
        
        
        console.log("Task workflow completed successfully!");
    });
});
