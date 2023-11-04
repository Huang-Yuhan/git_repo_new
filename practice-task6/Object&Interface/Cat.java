public class Cat extends Animal implements Behavior{
    
        public Cat(String name) {
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
    
}
