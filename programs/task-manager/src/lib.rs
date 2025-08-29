// lib.rs
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Mint};

declare_id!("TaskMgr111111111111111111111111111111111111");

#[program]
pub mod task_manager {
    use super::*;

    pub fn initialize_user(ctx: Context<InitializeUser>) -> Result<()> {
        let user_profile = &mut ctx.accounts.user_profile;
        user_profile.owner = ctx.accounts.user.key();
        user_profile.tasks_created = 0;
        user_profile.tasks_completed = 0;
        user_profile.reputation_score = 100;
        
        msg!("User profile initialized for: {}", ctx.accounts.user.key());
        Ok(())
    }

    pub fn create_task(
        ctx: Context<CreateTask>,
        task_id: u64,
        title: String,
        description: String,
        reward_amount: u64,
    ) -> Result<()> {
        let task = &mut ctx.accounts.task;
        let user_profile = &mut ctx.accounts.user_profile;
        
        task.id = task_id;
        task.creator = ctx.accounts.user.key();
        task.assignee = None;
        task.title = title;
        task.description = description;
        task.status = TaskStatus::Open;
        task.reward_amount = reward_amount;
        task.created_at = Clock::get()?.unix_timestamp;
        
        user_profile.tasks_created += 1;
        
        msg!("Task created with ID: {}", task_id);
        Ok(())
    }

    pub fn assign_task(ctx: Context<AssignTask>, assignee: Pubkey) -> Result<()> {
        let task = &mut ctx.accounts.task;
        
        require!(task.status == TaskStatus::Open, TaskError::TaskNotOpen);
        require!(task.creator == ctx.accounts.user.key(), TaskError::NotTaskCreator);
        
        task.assignee = Some(assignee);
        task.status = TaskStatus::Assigned;
        
        msg!("Task assigned to: {}", assignee);
        Ok(())
    }

    pub fn complete_task(ctx: Context<CompleteTask>) -> Result<()> {
        let task = &mut ctx.accounts.task;
        let assignee_profile = &mut ctx.accounts.assignee_profile;
        
        require!(task.status == TaskStatus::Assigned, TaskError::TaskNotAssigned);
        require!(
            Some(ctx.accounts.assignee.key()) == task.assignee,
            TaskError::NotAssignee
        );
        
        task.status = TaskStatus::Completed;
        task.completed_at = Some(Clock::get()?.unix_timestamp);
        assignee_profile.tasks_completed += 1;
        assignee_profile.reputation_score += 10;
        
        msg!("Task completed by: {}", ctx.accounts.assignee.key());
        Ok(())
    }

    pub fn claim_reward(ctx: Context<ClaimReward>) -> Result<()> {
        let task = &ctx.accounts.task;
        
        require!(task.status == TaskStatus::Completed, TaskError::TaskNotCompleted);
        require!(
            Some(ctx.accounts.assignee.key()) == task.assignee,
            TaskError::NotAssignee
        );
        
        // Cross Program Invocation - Mint reward tokens
        let mint_instruction = token::MintTo {
            mint: ctx.accounts.reward_mint.to_account_info(),
            to: ctx.accounts.assignee_token_account.to_account_info(),
            authority: ctx.accounts.mint_authority.to_account_info(),
        };
        
        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            mint_instruction,
        );
        
        token::mint_to(cpi_ctx, task.reward_amount)?;
        
        msg!("Reward claimed: {} tokens", task.reward_amount);
        Ok(())
    }
}

// Account Structures
#[account]
pub struct UserProfile {
    pub owner: Pubkey,
    pub tasks_created: u64,
    pub tasks_completed: u64,
    pub reputation_score: u64,
}

#[account]
pub struct Task {
    pub id: u64,
    pub creator: Pubkey,
    pub assignee: Option<Pubkey>,
    pub title: String,
    pub description: String,
    pub status: TaskStatus,
    pub reward_amount: u64,
    pub created_at: i64,
    pub completed_at: Option<i64>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum TaskStatus {
    Open,
    Assigned,
    Completed,
}

// Context Structures
#[derive(Accounts)]
pub struct InitializeUser<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + 32 + 8 + 8 + 8, // discriminator + pubkey + 3 u64s
        seeds = [b"user_profile", user.key().as_ref()],
        bump
    )]
    pub user_profile: Account<'info, UserProfile>,
    
    #[account(mut)]
    pub user: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(task_id: u64)]
pub struct CreateTask<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + 8 + 32 + 33 + 256 + 512 + 1 + 8 + 8 + 9, // Task account size
        seeds = [b"task", user.key().as_ref(), task_id.to_le_bytes().as_ref()],
        bump
    )]
    pub task: Account<'info, Task>,
    
    #[account(
        mut,
        seeds = [b"user_profile", user.key().as_ref()],
        bump
    )]
    pub user_profile: Account<'info, UserProfile>,
    
    #[account(mut)]
    pub user: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AssignTask<'info> {
    #[account(mut)]
    pub task: Account<'info, Task>,
    
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct CompleteTask<'info> {
    #[account(mut)]
    pub task: Account<'info, Task>,
    
    #[account(
        mut,
        seeds = [b"user_profile", assignee.key().as_ref()],
        bump
    )]
    pub assignee_profile: Account<'info, UserProfile>,
    
    pub assignee: Signer<'info>,
}

#[derive(Accounts)]
pub struct ClaimReward<'info> {
    pub task: Account<'info, Task>,
    
    #[account(mut)]
    pub reward_mint: Account<'info, Mint>,
    
    #[account(mut)]
    pub assignee_token_account: Account<'info, TokenAccount>,
    
    pub assignee: Signer<'info>,
    pub mint_authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

// Error Handling
#[error_code]
pub enum TaskError {
    #[msg("Task is not open")]
    TaskNotOpen,
    #[msg("Not the task creator")]
    NotTaskCreator,
    #[msg("Task is not assigned")]
    TaskNotAssigned,
    #[msg("Not the assignee")]
    NotAssignee,
    #[msg("Task is not completed")]
    TaskNotCompleted,
}
