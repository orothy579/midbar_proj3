# Tic Tac Toe Pseudo Code

## 1. 초기화

- board: 3x3 배열(각 칸은 비어있는 상태로 시작)
- currentPlayer: player1
- gameEnd: false

## 2. 메인 게임 루프

WHILE gameEnd == false:
    # 2.1 보드 출력
    printBoard(board)

    # 2.2 현재 플레이어로부터 입력 받기
    row, col = inputFromPlayer(currentPlayer)  # 사용자로부터 행(row), 열(col) 입력

    # 2.3 입력 유효성 검사
    IF positionIsValid(board, row, col) == false:
        print("해당 위치에 둘 수 없습니다. 다시 입력해주세요.")
        CONTINUE  # 이 턴을 다시 진행하도록 넘어감(현재 플레이어 변경 없이 반복)

    # 2.4 유효하다면 보드에 표시
    board[row][col] = markOf(currentPlayer)  # 현재 플레이어에 맞는 O 또는 X 등 마크를 기입

    # 2.5 승리 조건 체크
    IF checkWin(board, currentPlayer) == true:
        printBoard(board)
        print(currentPlayer + " 승리!")
        gameEnd = true
        BREAK

    # 2.6 무승부(보드가 가득 참) 체크
    IF checkBoardFull(board) == true:
        printBoard(board)
        print("무승부입니다.")
        gameEnd = true
        BREAK

    # 2.7 플레이어 교체
    IF currentPlayer == player1:
        currentPlayer = player2
    ELSE:
        currentPlayer = player1
END WHILE

## 3. 게임 종료 후

print("게임이 종료되었습니다.")
