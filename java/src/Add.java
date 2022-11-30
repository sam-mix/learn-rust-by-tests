public class Add {
    public int add(int a, int b) {
        System.out.printf("a = %x, b = %x\n", a, b);
        return b == 0 ? a : add(a ^ b, (a & b) << 1);
    }
    


    public static void main(String[] args) {
        var a = new Add();

        //   00000001
        //   00000011
        // ^ 00000010

        //   00000001
        //   00000011
        // & 00000001

        //   00000010
        //   00000010
        // ^ 00000000

        //   00000010
        //   00000010
        // & 00000010

        //   00000000
        //   00000100
        // ^ 00000100

        //   00000011
        //   00000100
        // & 00000000



        System.out.println(a.add(1, 3));
        
    }
}
