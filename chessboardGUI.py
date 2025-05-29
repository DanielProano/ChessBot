import pygame
import time

pygame.init()

WIDTH = 600
HEIGHT = 600
SQ_SIZE = WIDTH // 8
WHITE = (255, 255, 255)
BLACK = (0, 0, 0)
BLUE = (0, 0, 255)

# Timer settings
WHITE_TIME = 300  # 5 minutes for white player
BLACK_TIME = 300  # 5 minutes for black player
font = pygame.font.SysFont('Arial', 30)


# Chess pieces (just simple rectangles for now)
class Piece:
    def __init__(self, color, x, y):
        self.color = color
        self.x = x
        self.y = y
        self.rect = pygame.Rect(x * SQ_SIZE, y * SQ_SIZE, SQ_SIZE, SQ_SIZE)

    def draw(self, win):
        pygame.draw.rect(win, self.color, self.rect)


# Create an empty board with pieces
def create_board():
    board = []
    for i in range(8):
        board.append([None] * 8)

    # Add pieces (just pawns for now)
    for i in range(8):
        board[1][i] = Piece(WHITE, i, 1)
        board[6][i] = Piece(BLACK, i, 6)

    return board


# Draw the board and pieces
def draw_board(win, board):
    win.fill(BLACK)  # Fill the background with black to make sure it's clean
    for row in range(8):
        for col in range(8):
            # Alternate between white and black squares
            color = WHITE if (row + col) % 2 == 0 else BLACK
            pygame.draw.rect(win, color, (col * SQ_SIZE, row * SQ_SIZE, SQ_SIZE, SQ_SIZE))
            piece = board[row][col]
            if piece:
                piece.draw(win)
    pygame.display.update()


# Draw the timer
def draw_timer(win, white_time, black_time, current_turn):
    white_minutes = white_time // 60
    white_seconds = white_time % 60
    black_minutes = black_time // 60
    black_seconds = black_time % 60

    white_time_text = font.render(f"White: {white_minutes:02}:{white_seconds:02}", True, WHITE)
    black_time_text = font.render(f"Black: {black_minutes:02}:{black_seconds:02}", True, WHITE)

    win.blit(white_time_text, (50, 10))
    win.blit(black_time_text, (WIDTH - 150, 10))

    pygame.display.update()


# Main game loop
def main():
    win = pygame.display.set_mode((WIDTH, HEIGHT))
    pygame.display.set_caption("Chess Game")

    board = create_board()

    running = True
    clock = pygame.time.Clock()

    current_turn = WHITE  # White starts first
    last_time = time.time()
    white_time_left = WHITE_TIME
    black_time_left = BLACK_TIME

    selected_piece = None
    dragging_piece = False

    while running:
        clock.tick(60)  # 60 FPS
        win.fill(BLACK)
        draw_board(win, board)

        # Timer countdown
        now = time.time()
        if current_turn == WHITE:
            white_time_left -= int(now - last_time)
        else:
            black_time_left -= int(now - last_time)

        last_time = now
        draw_timer(win, white_time_left, black_time_left, current_turn)

        # Event handling
        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                running = False

            if event.type == pygame.MOUSEBUTTONDOWN:
                x, y = event.pos
                col, row = x // SQ_SIZE, y // SQ_SIZE
                piece = board[row][col]
                if piece and piece.color == current_turn:
                    selected_piece = piece
                    dragging_piece = True

            if event.type == pygame.MOUSEBUTTONUP:
                if dragging_piece and selected_piece:
                    x, y = pygame.mouse.get_pos()
                    col, row = x // SQ_SIZE, y // SQ_SIZE

                    # Move the piece to the new position
                    board[selected_piece.y][selected_piece.x] = None
                    selected_piece.x = col
                    selected_piece.y = row
                    board[row][col] = selected_piece

                    # Switch turn
                    current_turn = WHITE if current_turn == BLACK else BLACK
                    dragging_piece = False
                    selected_piece = None

            if event.type == pygame.MOUSEMOTION:
                if dragging_piece and selected_piece:
                    x, y = event.pos
                    selected_piece.rect.topleft = (x - SQ_SIZE // 2, y - SQ_SIZE // 2)

        pygame.display.update()

    pygame.quit()


if __name__ == "__main__":
    main()