public class Dog extends Animal implements Behavior {

    public Dog(String name) {
        super(name);
    }

    @Override
    public void run() {
        System.out.println(name + " is running");
    }

    @Override
    public void sleep() {
        System.out.println(name + " is sleeping");
    }

    @Override
    public void say() {
        System.out.println(name);
    }
}