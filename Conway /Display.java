import java.awt.*;
import java.awt.event.*;
import java.util.LinkedList;
import java.util.*;
import java.util.concurrent.*;


public class Display extends Canvas{
    // this will have the display and stuff
    static final Color cellColor = new Color(255, 131, 131);
    static final Color lineColor = new Color(150,150,255);

    public LinkedList<Cell> getNeighbourCells(Cell c){
        LinkedList<Cell> result = new LinkedList<>();
        // positions are just -1, 0, 1
        for (int x = -1; x <= 1; x++){
            for (int y = -1; y <= 1; y++){
                // if it's not the cell at the very center, which is cell c (NOT A NEIGHBOR!!!)
                // also not outside of the bounds of the board.
                int newX = c.x + x;
                int newY = c.y+y;
                if ( !(newX == c.x && newY == c.y) && (newX>=0 && newX<Main._XSIZE) && (newY>=0 && newY<Main._YSIZE) ){
                    Cell foundCell = Main.board[newX][newY];
                    // we will account for living cells only.
                    if (foundCell.getAlive()){
                        result.add(Main.board[newX][newY]);
                    }
                    //
                }
                // and this gets the live cells yea
            }
        }

        return result;
    }

    public void updateBoard() {
        // first rule (and 3rd); any cell that has less than 2 live neighbors will die
        // and any cell with over 3 neighbors will also die
        for(int x = 0; x<Main._XSIZE;x++) {
            for (int y = 0; y < Main._YSIZE; y++) {
                Cell foundCell = Main.board[x][y];
                // with that cell let's check its neighbors.
                LinkedList<Cell> liveNeighbors = getNeighbourCells(foundCell);
                if (foundCell.getAlive() && liveNeighbors.size() < 2) {
                    //System.out.println("You suffer from underpopulation. Avada Kedavra! " + liveNeighbors.size());
                    foundCell.die();
                }
                //
                if (foundCell.getAlive() && liveNeighbors.size() > 3){
                    //System.out.println("You suffer from overpopulation. Avada Kedavra! " + liveNeighbors.size());
                    foundCell.die();
                }
                // 4th rule, we can rez any cell with exactly 3 live neighbors.
                if (!foundCell.getAlive() && liveNeighbors.size() == 3){
                    //System.out.println("Come back!");
                    foundCell.resurrect();
                }
                // if you didn't meet any of these you live!
            }
        }

    }

    @Override
    public void paint(Graphics g){
        // update the board
        super.paint(g);

        updateBoard();

        //   ================================================== DISPLAY PART ==========================================
        setBackground(Color.BLACK);

        //draw some cells and shit. :)
        g.setColor(cellColor);
        int xScale = (Main._BOARDSIZEX/Main._XSIZE);
        int yScale = (Main._BOARDSIZEY/Main._YSIZE);

        for(int x = 0; x<Main._XSIZE;x++){
            for(int y = 0; y<Main._YSIZE;y++){
                // we will draw a cell here.
              Cell foundCell = Main.board[x][y];

                // if this cell is alive, we'll fill a rectangle
                // otherwise we'll just clear it because they are dead
                if (foundCell.getAlive()){
                    g.fillRect(x*xScale,y*yScale,xScale,yScale);

                }else{
                    g.clearRect(x*xScale,y*yScale,xScale,yScale);
                }

            }

        }
        if (Main._INCLUDELINES){
            // we can draw the lines this way i think. sorta, kinda, partially.
            g.setColor(lineColor);

            for(int x = 0; x<=Main._XSIZE;x++){
                int xPos = x*xScale;
                // we draw an x line here ^_^
                g.drawLine(xPos,0,xPos,Main._BOARDSIZEY);
            }
            for(int y = 0; y<=Main._YSIZE;y++){
                int yPos = y*yScale;
                g.drawLine(0,yPos,Main._BOARDSIZEX,yPos);
            }

        }

    }

    @Override
    public void update(Graphics g){

        paint(g);
    }
}
