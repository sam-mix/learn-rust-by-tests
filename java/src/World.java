
public class World {
    private String name; // 名称
    private Integer age; // 年龄

    public World() {
    }

    public World(String name, Integer age) {
        this.name = name;
        this.age = age;
    }

    public String getName() {
        return name;
    }

    public void setName(String name) {
        this.name = name;
    }

    public Integer getAge() {
        return age;
    }

    public void setAge(Integer age) {
        this.age = age;
    }

    @Override
    public int hashCode() {
        final int prime = 31;
        int result = 1;
        result = prime * result + ((name == null) ? 0 : name.hashCode());
        result = prime * result + ((age == null) ? 0 : age.hashCode());
        return result;
    }

    @Override
    public boolean equals(Object obj) {
        if (this == obj)
            return true;
        if (obj == null)
            return false;
        if (getClass() != obj.getClass())
            return false;
        World other = (World) obj;
        if (name == null) {
            if (other.name != null)
                return false;
        } else if (!name.equals(other.name))
            return false;
        if (age == null) {
            if (other.age != null)
                return false;
        } else if (!age.equals(other.age))
            return false;
        return true;
    }

    @Override
    public String toString() {
        return "World [name=" + name + ", age=" + age + "]";
    }

    public static void main(String[] args) {
        System.out.println(new World().equals(null));
        System.out.println(new World().equals(1));
        System.out.println(new World().equals(new World("", 0)));
        System.out.println(new World().equals(new World(null, null)));
        System.out.println(new World());
    }
}
