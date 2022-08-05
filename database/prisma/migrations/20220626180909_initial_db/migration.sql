-- CreateEnum
CREATE TYPE "Outcome" AS ENUM ('WIN', 'LOSS', 'DRAW', 'UNDECIDED');

-- CreateTable
CREATE TABLE "User" (
    "id" TEXT NOT NULL,
    "startedPlaying" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "elo" INTEGER NOT NULL DEFAULT 1000,

    CONSTRAINT "User_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "Game" (
    "id" TEXT NOT NULL,
    "whiteId" TEXT NOT NULL,
    "blackId" TEXT NOT NULL,
    "whiteOutcome" "Outcome" NOT NULL,
    "blackOutcome" "Outcome" NOT NULL,
    "whiteElo" INTEGER NOT NULL,
    "blackElo" INTEGER NOT NULL,
    "initalBoard" TEXT NOT NULL,
    "currentBoard" TEXT NOT NULL,
    "moveList" TEXT[],

    CONSTRAINT "Game_pkey" PRIMARY KEY ("id")
);

-- AddForeignKey
ALTER TABLE "Game" ADD CONSTRAINT "Game_whiteId_fkey" FOREIGN KEY ("whiteId") REFERENCES "User"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "Game" ADD CONSTRAINT "Game_blackId_fkey" FOREIGN KEY ("blackId") REFERENCES "User"("id") ON DELETE RESTRICT ON UPDATE CASCADE;
