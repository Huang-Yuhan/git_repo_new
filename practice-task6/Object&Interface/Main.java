class Main
{
    public static void main(String[] args)
    {
        Dog dog = new Dog("Dog");
        Cat cat = new Cat("Cat");

        dog.say();
        dog.run();
        dog.sleep();

        cat.say();
        cat.run();
        cat.sleep();
    }
}