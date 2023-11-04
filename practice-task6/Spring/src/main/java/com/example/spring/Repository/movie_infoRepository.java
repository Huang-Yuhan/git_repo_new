package com.example.spring.Repository;

import com.example.spring.Entity.MovieInfoEntity;
import org.springframework.data.jpa.repository.JpaRepository;

public interface movie_infoRepository extends JpaRepository<MovieInfoEntity,Integer> {
    public MovieInfoEntity getByName(String name);
}
