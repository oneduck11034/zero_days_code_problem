public class Calc(x: Int, y: Int){
    val x: Int;
    val y: Int;

    init{
        this.x= x;
        this.y= y
    }

    fun add(): Int{
        return this.x + this.y;
    }
    fun del(): Int{
        return this.x - this.y;
    }
}

fun main(){
    val cal1= Calc(2,1,);
    val added= cal1.add();
    val deled= cal1.del();
    println("add: "+added);
    println("del: "+deled);
}
