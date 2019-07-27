import java.util.Scanner;
import java.lang.StringBuilder;


class Main{
    public static void main(String args[]){
        
        Scanner input = new Scanner(System.in);
        StringBuilder str = new StringBuilder();
        int loops = input.nextInt();

        for(int i = 1; i <= loops; i++){
            str.append(i);
            str.append(" Abracadabra\n");
        }

        System.out.println(str.toString());
    }
}
