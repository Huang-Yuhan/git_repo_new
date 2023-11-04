package com.example.spring.Repository;

import com.example.spring.Entity.UserEntity;
import org.springframework.data.jpa.repository.JpaRepository;

public interface userRepository extends JpaRepository<UserEntity, Integer>{
    public boolean existsByUsernameAndAndPassword(String username, String password);
}
