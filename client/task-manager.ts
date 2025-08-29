
import * as anchor from '@project-serum/anchor';
import { PublicKey, SystemProgram, LAMPORTS_PER_SOL } from '@solana/web3.js';
import { TOKEN_PROGRAM_ID, createMint, createAccount } from '@solana/spl-token';

export class TaskManagerClient {
    private program: anchor.Program;
    private provider: anchor.AnchorProvider;

    constructor(program: anchor.Program) {
        this.program = program;
        this.provider = program.provider as anchor.AnchorProvider;
    }

    async initializeUser(): Promise<string> {
        const userProfilePDA = this.getUserProfilePDA(
            this.provider.wallet.publicKey
        );

        const tx = await this.program.methods
            .initializeUser()
            .accounts({
                userProfile: userProfilePDA,
                user: this.provider.wallet.publicKey,
                systemProgram: SystemProgram.programId,
            })
            .rpc();

        console.log("User initialized. Transaction:", tx);
        return tx;
    }

    async createTask(
        taskId: number,
        title: string,
        description: string,
        rewardAmount: number
    ): Promise<string> {
        const taskPDA = this.getTaskPDA(
            this.provider.wallet.publicKey,
            taskId
        );
        const userProfilePDA = this.getUserProfilePDA(
            this.provider.wallet.publicKey
        );

        const tx = await this.program.methods
            .createTask(
                new anchor.BN(taskId),
                title,
                description,
                new anchor.BN(rewardAmount)
            )
            .accounts({
                task: taskPDA,
                userProfile: userProfilePDA,
                user: this.provider.wallet.publicKey,
                systemProgram: SystemProgram.programId,
            })
            .rpc();

        console.log("Task created. Transaction:", tx);
        return tx;
    }

    async assignTask(taskCreator: PublicKey, taskId: number, assignee: PublicKey): Promise<string> {
        const taskPDA = this.getTaskPDA(taskCreator, taskId);

        const tx = await this.program.methods
            .assignTask(assignee)
            .accounts({
                task: taskPDA,
                user: this.provider.wallet.publicKey,
            })
            .rpc();

        console.log("Task assigned. Transaction:", tx);
        return tx;
    }

    async completeTask(taskCreator: PublicKey, taskId: number): Promise<string> {
        const taskPDA = this.getTaskPDA(taskCreator, taskId);
        const assigneeProfilePDA = this.getUserProfilePDA(
            this.provider.wallet.publicKey
        );

        const tx = await this.program.methods
            .completeTask()
            .accounts({
                task: taskPDA,
                assigneeProfile: assigneeProfilePDA,
                assignee: this.provider.wallet.publicKey,
            })
            .rpc();

        console.log("Task completed. Transaction:", tx);
        return tx;
    }

    private getUserProfilePDA(userPubkey: PublicKey): PublicKey {
        const [pda] = PublicKey.findProgramAddressSync(
            [Buffer.from("user_profile"), userPubkey.toBuffer()],
            this.program.programId
        );
        return pda;
    }

    private getTaskPDA(userPubkey: PublicKey, taskId: number): PublicKey {
        const [pda] = PublicKey.findProgramAddressSync(
            [
                Buffer.from("task"),
                userPubkey.toBuffer(),
                Buffer.from(taskId.toString())
            ],
            this.program.programId
        );
        return pda;
    }

  
    async getUserProfile(userPubkey: PublicKey) {
        const profilePDA = this.getUserProfilePDA(userPubkey);
        return await this.program.account.userProfile.fetch(profilePDA);
    }

    async getTask(taskCreator: PublicKey, taskId: number) {
        const taskPDA = this.getTaskPDA(taskCreator, taskId);
        return await this.program.account.task.fetch(taskPDA);
    }
}
