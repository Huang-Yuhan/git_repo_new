package com.example.spring.Repository;

import com.example.spring.Entity.CommentEntity;
import org.springframework.data.jpa.repository.JpaRepository;

public interface commentRepository extends JpaRepository<CommentEntity, Integer> {

}
